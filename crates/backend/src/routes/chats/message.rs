use std::{sync::Arc, time::Duration};

use ai::openai::{
    client::OpenAIClient,
    completions::{
        OpenAICompletionDelta, OpenAIMessage, OpenAIMessageContent, OpenAIMessageContentFile,
        OpenAIMessageImageUrl, OpenRouterRequestPdfPlugin, OpenRouterRequestPlugin,
        ReasoningEffort,
    },
};
use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::Utc;
use futures::{AsyncReadExt, StreamExt, TryStreamExt, future::join_all};
use model::{
    key::UserApiKey,
    memory::Memory,
    message::{ChatMessage, ChatMessageContent, Role},
    upload::UserUpload,
};
use mongodb::bson::{Bson, doc, oid::ObjectId};
use redis_om::HashModel;
use search::WebSearchOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::{
        chat::{ChatMessageContentPayload, ChatMessagePayload},
        memories::MemoryPayload,
    },
    state::{AppState, inference::InferenceProvider},
    streaming::{ApiDelta, ControlChunk},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompletionPayload {
    pub message: String,
    pub model: String,
    pub reasoning: Option<ReasoningEffort>,
    pub use_search: bool,
    pub use_memories: bool,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<ObjectId>,
    Auth(session): Auth,
    Json(payload): Json<PromptCompletionPayload>,
) -> Result<impl IntoResponse, ApplicationError> {
    let model = state
        .models()
        .free_models()
        .iter()
        .find(|model| model.identifier == payload.model)
        .cloned()
        .or_else(|| {
            state
                .models()
                .paid_models()
                .iter()
                .find(|model| model.identifier == payload.model)
                .cloned()
        })
        .ok_or(ApplicationError::InvalidModelIdentifier)?;

    let chat = state
        .storage()
        .database()
        .chats
        .get(doc! { "_id": chat_id, "user_id": session.user_id })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;
    let Some(chat) = chat else {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::ChatDoesNotExist,
        )));
    };

    // MESSAGES

    let messages = state
        .storage()
        .database()
        .messages
        .get_many_sorted(
            doc! { "chat_id": chat.id.unwrap() },
            doc! { "timestamp": -1 },
        )
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let mut messages = messages
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|msg| {
            msg.map(|msg| OpenAIMessage {
                content: msg
                    .content
                    .into_iter()
                    .map(|content| match content {
                        ChatMessageContent::Text { value } => {
                            OpenAIMessageContent::Text { text: value }
                        }
                        ChatMessageContent::Image { id } => OpenAIMessageContent::ImageUrl {
                            image_url: OpenAIMessageImageUrl {
                                url: format!(
                                    "https://t3-chat-clone.onrender.com/files/{}/{}",
                                    chat_id.to_hex(),
                                    id.to_hex()
                                ),
                            },
                        },
                        ChatMessageContent::Pdf { id } => OpenAIMessageContent::Text {
                            text: format!("**pdf file with id: {id}**"),
                        },
                    })
                    .collect(),
                role: msg.role.to_string(),
            })
        })
        .collect::<Result<Vec<_>, mongodb::error::Error>>()
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    // FILES

    let files_chat_id = if messages.is_empty() {
        None
    } else {
        Some(chat.id.unwrap())
    };
    let files = state
        .storage()
        .database()
        .uploads
        .get_many(doc! { "user_id": session.user_id, "chat_id": files_chat_id, "is_sent": false })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let files = files.try_collect::<Vec<UserUpload>>().await.map_err(|e| {
        ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
            anyhow!(e),
        )))
    })?;

    for file in files.iter() {
        state
            .storage()
            .database()
            .uploads
            .update(
                file.id,
                doc! { "$set": { "chat_id": chat.id.unwrap(), "is_sent": true } },
            )
            .await
            .map_err(|e| {
                ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                    e,
                )))
            })?;
    }

    // API KEY

    let mut conn = state.storage().cache().connection();

    let api_key = if let Ok(cached_key) =
        UserApiKey::get(format!("openrouter-{}", session.user_id), &mut conn).await
    {
        Some(cached_key.key)
    } else {
        let key = state
            .storage()
            .database()
            .keys
            .get(doc! { "user_id": session.user_id })
            .await
            .unwrap();
        if let Some(ref key) = key {
            let _ = UserApiKey {
                id: format!("openrouter-{}", session.user_id),
                key: key.key.clone(),
                key_id: key.id.unwrap().to_hex(),
            }
            .save(&mut conn)
            .await;
        }

        key.map(|key| key.key)
    };

    let client = if let Some(api_key) = api_key {
        // currently, only OpenRouter API keys are supported
        OpenAIClient::new(api_key, "https://openrouter.ai/api".to_string())
    } else {
        match model.provider {
            InferenceProvider::OpenRouter => state.inference().openrouter.clone(),
            InferenceProvider::Chutes => state.inference().chutes.clone(),
        }
    };

    let (tx, rx) = flume::unbounded();

    let memories = if payload.use_memories {
        state
            .storage()
            .database()
            .memories
            .get_many(doc! { "user_id": session.user_id })
            .await
            .unwrap()
            .map_ok(|memory| memory.content)
            .try_collect::<Vec<String>>()
            .await
            .map_err(|e| {
                ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                    anyhow!(e),
                )))
            })?
    } else {
        vec![]
    };

    let mut user_message_full_content = vec![ChatMessageContent::Text {
        value: payload.message.clone(),
    }];
    for file in files.iter() {
        if file.content_type.starts_with("image/") {
            user_message_full_content.push(ChatMessageContent::Image { id: file.id })
        } else {
            user_message_full_content.push(ChatMessageContent::Pdf { id: file.id });
        }
    }

    let user_message = ChatMessage {
        id: None,
        content: user_message_full_content.clone(),
        model: None,
        reasoning: None,
        role: Role::User,
        chat_id: chat.id.unwrap(),
        updated_memory: None,
        timestamp: Utc::now(),
    };

    if messages.is_empty() {
        let title_generation_message = format!(
            "Here are some examples of first messages and their chat names:\n\ninput: I need help choosing a new laptop for college.\noutput: Laptop Recommendations for College\n\ninput:  Best places to eat Italian food in downtown Chicago?\noutput: Chicago Italian Food Guide\n\nNow, generate a descriptive name for a chat where the first message was: \"{}\"\nYour output must be a SINGLE, SHORT sentence. Do not include any parentheses, other symbols or any words except for the final result.",
            payload.message
        );

        let task_state = Arc::clone(&state);
        let task_tx = tx.clone();
        tokio::spawn(async move {
            let chat_name = task_state
                .inference()
                .chutes
                .clone()
                .prompt_completion_non_streaming(
                    "chutesai/Mistral-Small-3.1-24B-Instruct-2503".to_string(),
                    title_generation_message,
                    Some(0.),
                    Some(1000),
                )
                .await
                .unwrap();

            let _ = task_tx
                .send_async(ApiDelta::Control(ControlChunk::ChatNameUpdated {
                    name: chat_name.clone(),
                }))
                .await;

            task_state
                .storage()
                .database()
                .chats
                .update(chat.id.unwrap(), doc! { "$set": { "name": chat_name } })
                .await
                .unwrap();
        });
    }

    let user_message_id = state
        .storage()
        .database()
        .messages
        .create(user_message.clone())
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let stream_id = Uuid::new_v4();
    let task_state = Arc::clone(&state);
    tokio::spawn(async move {
        let search_results = if payload.use_search {
            let search_query = payload.message.trim();
            if search_query.is_empty() || search_query.len() > 400 {
                None
            } else {
                task_state
                    .search()
                    .search(WebSearchOptions {
                        language: "en".to_string(),
                        query: search_query.to_string(),
                        region: "us".to_string(),
                    })
                    .await
                    .ok()
            }
        } else {
            None
        };

        tracing::debug!("Searched");

        tx.send_async(ApiDelta::Control(ControlChunk::WebSearchPerformed))
            .await
            .unwrap();

        let mut user_message_content = user_message.content.clone();
        let user_message_text = if payload.use_memories {
            format!(
                "If necessary, you may use the following memories about the user to answer: {};\n{}",
                if memories.is_empty() {
                    "No memories yet.".to_string()
                } else {
                    serde_json::to_string(&memories).unwrap()
                },
                match &user_message_content[0] {
                    ChatMessageContent::Text { value } => value,
                    _ => unreachable!(),
                }
            )
        } else {
            match &user_message_content[0] {
                ChatMessageContent::Text { value } => value.to_string(),
                _ => unreachable!(),
            }
        };

        user_message_content[0] = ChatMessageContent::Text {
            value: user_message_text,
        };

        let message_for_assistant = if let Some(results) = search_results {
            let context: String = results
                .into_iter()
                .take(10)
                .map(|result| {
                    format!(
                        " - Title: {};\nSnippet: {};\nSource: {};\n",
                        result.title, result.snippet, result.link
                    )
                })
                .collect();

            let mut user_message_content = user_message_content;

            let prompt_content = format!(
                r#"Use the following search results to answer the query. If information is insufficient, state that.;
              Search Results:\n{context};
              Query: {};
              \nAnswer:"#,
                match &user_message.content[0] {
                    ChatMessageContent::Text { value } => value,
                    _ => unreachable!(),
                }
                .trim()
            );

            let new_first_message = ChatMessageContent::Text {
                value: prompt_content,
            };
            user_message_content[0] = new_first_message;

            user_message_content
        } else {
            user_message_content
        };

        messages.push(OpenAIMessage {
            role: "user".to_string(),
            content: join_all(
                message_for_assistant
                    .into_iter()
                    .map(async |msg| match msg {
                        ChatMessageContent::Text { value } => {
                            OpenAIMessageContent::Text { text: value }
                        }
                        ChatMessageContent::Image { id } => {
                            // if cfg!(debug_assertions) {
                            let mut file = task_state
                                .storage()
                                .bucket()
                                .gridfs()
                                .open_download_stream(Bson::ObjectId(id))
                                .await
                                .unwrap();
                            let mut contents = vec![];
                            file.read_to_end(&mut contents).await.unwrap();
                            OpenAIMessageContent::ImageUrl {
                                image_url: OpenAIMessageImageUrl {
                                    url: format!(
                                        "data:image/jpeg;base64,{}",
                                        BASE64_STANDARD.encode(contents)
                                    ),
                                },
                            }
                            // } else {
                            //     OpenAIMessageContent::ImageUrl {
                            //         image_url: OpenAIMessageImageUrl {
                            //             url: format!(
                            //                 "https://t3-chat-clone.onrender.com/files/{}/{}",
                            //                 chat_id.to_hex(),
                            //                 id.to_hex()
                            //             ),
                            //         },
                            //     }
                            // }
                        }
                        ChatMessageContent::Pdf { id } => {
                            let mut file = task_state
                                .storage()
                                .bucket()
                                .gridfs()
                                .open_download_stream(Bson::ObjectId(id))
                                .await
                                .unwrap();
                            let mut contents = vec![];
                            file.read_to_end(&mut contents).await.unwrap();
                            OpenAIMessageContent::File {
                                file: OpenAIMessageContentFile {
                                    filename: id.to_hex(),
                                    file_data: format!(
                                        "data:application/pdf;base64,{}",
                                        BASE64_STANDARD.encode(contents)
                                    ),
                                },
                            }
                        }
                    }),
            )
            .await,
        });

        let assistant_message_id = ObjectId::new();
        let assistant_message = ChatMessage {
            id: Some(assistant_message_id),
            content: vec![],
            model: Some(model.name.clone()),
            role: Role::Assistant,
            reasoning: None,
            updated_memory: None,
            chat_id: chat.id.unwrap(),
            timestamp: Utc::now(),
        };

        let task2_state = Arc::clone(&task_state);
        let task_message = assistant_message.clone();
        let task_tx = tx.clone();
        let task_memories = memories.clone();
        tokio::spawn(async move {
            task2_state
                .storage()
                .database()
                .messages
                .create(task_message)
                .await
                .unwrap();

            if !payload.use_memories {
                return;
            }
            tokio::spawn(async move {
                let prompt = format!("You are an AI Memory Assistant. Your task is to:
1. Analyze the current user message.
2. If there is an existing memory in [Existing memories] that directly pertains to the message, output NONE.
3. If no relevant memory exists, but the message contains important information (e.g., goals, preferences, or facts), generate a new concise memory statement.
4. If there are multiple memories you see in a single message, output the most important one.
5. Otherwise, output NONE.

Memory format (do NOT include braces): [Concise memory statement, single sentence]. Do not include any additional tokens in the output, except for the memory.
For example: Input - Hey there! I am building an AI chat., Output - User is building an AI chat.

Existing memories: {};

Current user message: {:?}
Your output:", if task_memories.is_empty() { "No memories yet".to_string() } else { serde_json::to_string(&task_memories).unwrap() }, payload.message.trim());

                let memory = task2_state
                    .inference()
                    .chutes
                    .clone()
                    .prompt_completion_non_streaming(
                        "chutesai/Mistral-Small-3.1-24B-Instruct-2503".to_string(),
                        prompt,
                        Some(0.3),
                        Some(1000),
                    )
                    .await
                    .unwrap();

                if memory.trim().starts_with("NONE") {
                    return;
                }

                let memory = if memory.starts_with('[') {
                    &memory[1..]
                } else {
                    &memory
                };
                let memory = if memory.ends_with(']') {
                    &memory[..(memory.len() - 1)]
                } else if memory.ends_with("].") {
                    &format!("{}.", &memory[..(memory.len() - 2)])
                } else {
                    memory
                };

                let memory = memory.to_string();

                let memory_id = task2_state
                    .storage()
                    .database()
                    .memories
                    .create(Memory {
                        id: None,
                        user_id: session.user_id,
                        content: memory.clone(),
                    })
                    .await
                    .unwrap();
                task2_state
                    .storage()
                    .database()
                    .messages
                    .update(
                        assistant_message_id,
                        doc! { "$set": { "updated_memory": memory.clone() } },
                    )
                    .await
                    .unwrap();

                let _ = task_tx
                    .send_async(ApiDelta::Control(ControlChunk::MemoryAdded {
                        memory: MemoryPayload {
                            id: memory_id,
                            content: memory,
                        },
                    }))
                    .await;
            });
        });

        let stream = client
            .completion(
                payload.model,
                messages,
                Some(0.7),
                payload.reasoning,
                vec![OpenRouterRequestPlugin {
                    id: "file-parser".to_string(),
                    pdf: OpenRouterRequestPdfPlugin {
                        engine: "pdf-text".to_string(),
                    },
                }],
            )
            .await;
        let Ok(stream) = stream else {
            let error = stream.err().unwrap();
            tracing::error!("Failed to get stream: {}", error);
            if let Ok(code) = error.downcast::<StatusCode>() {
                let _ = tx
                    .send_async(ApiDelta::Control(ControlChunk::InferenceError {
                        code: code.as_u16(),
                    }))
                    .await;
            }
            return;
        };
        tracing::debug!("Created stream.");
        let mut stream = Box::pin(stream);

        let mut reasoning: Option<String> = None;
        let mut content = String::new();

        let mut reasoning_acc: Option<String> = None;
        let mut content_acc = String::new();
        let mut iteration_start = Utc::now().timestamp_millis();

        while let Ok(chunk) = stream.try_next().await {
            let Some(chunk) = chunk else { break };
            let delta = &chunk.choices.get(0).unwrap().delta;
            let reasoning_content = delta.reasoning.as_ref();
            let delta_content = delta.content.as_ref();

            if let Some(delta_content) = delta_content {
                content_acc.push_str(delta_content);
                content.push_str(delta_content);
            }

            if let Some(reasoning_content) = reasoning_content {
                if let Some(ref mut reasoning) = reasoning {
                    reasoning.push_str(reasoning_content);
                } else {
                    reasoning = Some(reasoning_content.to_string())
                }

                if reasoning_acc.is_none() {
                    reasoning_acc = Some(reasoning_content.to_string());
                } else {
                    reasoning_acc.as_mut().unwrap().push_str(reasoning_content);
                }
            }

            if Utc::now().timestamp_millis() - iteration_start >= 100 {
                tx.send_async(ApiDelta::Chunk(OpenAICompletionDelta {
                    content: Some(content_acc.clone()),
                    reasoning: reasoning_acc.take(),
                    role: Some("assistant".to_string()),
                }))
                .await
                .unwrap();
                content_acc = String::new();
                iteration_start = Utc::now().timestamp_millis();
            }
        }
        if !content_acc.is_empty() || reasoning_acc.is_some() {
            tx.send_async(ApiDelta::Chunk(OpenAICompletionDelta {
                content: Some(content_acc.clone()),
                reasoning: reasoning_acc.take(),
                role: Some("assistant".to_string()),
            }))
            .await
            .unwrap();
        }
        tracing::debug!("Sending done chunk");
        let assistant_message_content = vec![ChatMessageContent::Text { value: content }];
        tx.send(ApiDelta::Control(ControlChunk::Done {
            message: ChatMessage {
                content: assistant_message_content.clone(),
                reasoning: reasoning.clone(),
                ..assistant_message
            },
        }))
        .unwrap();
        task_state
            .storage()
            .database()
            .messages
            .update(
                assistant_message_id,
                doc! { "$set": { "content": assistant_message_content, "reasoning": reasoning } },
            )
            .await
            .unwrap();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(20)).await;
            if task_state.remove_stream(&stream_id) {
                tracing::debug!("Streaming terminated due to inactivity.")
            }
        });
    });

    state.insert_stream(stream_id, rx);

    let content = user_message_full_content
        .into_iter()
        .map(|message| match message {
            ChatMessageContent::Text { value } => ChatMessageContentPayload::Text { value },
            ChatMessageContent::Image { id } => ChatMessageContentPayload::Image { id },
            ChatMessageContent::Pdf { id } => ChatMessageContentPayload::Pdf { id },
        })
        .collect();

    Ok((
        StatusCode::OK,
        Json(json!({
          "stream_id": stream_id,
          "user_message": ChatMessagePayload {
            id: user_message_id,
            chat_id: user_message.chat_id,
            content,
            model: None,
            reasoning: None,
            updated_memory: None,
            role: user_message.role,
            timestamp: user_message.timestamp
          }
        })),
    )
        .into_response())
}
