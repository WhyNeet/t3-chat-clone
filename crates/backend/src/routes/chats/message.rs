use std::{env, str::FromStr, sync::Arc, time::Duration};

use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::AeadMut};
use ai::openai::{
    completions::{
        OpenAICompletionDelta, OpenAIMessage, OpenAIMessageContent, OpenAIMessageContentFile,
        OpenAIMessageImageUrl, OpenRouterRequestPdfPlugin, OpenRouterRequestPlugin,
        ReasoningEffort,
    },
    streaming::OpenAIClient,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::Utc;
use futures::{AsyncReadExt, StreamExt, TryStreamExt, future::join_all};
use hmac::digest::generic_array::GenericArray;
use model::{
    key::UserApiKey,
    message::{ChatMessage, ChatMessageContent, Role},
    upload::UserUpload,
};
use mongodb::bson::{Bson, doc, oid::ObjectId};
use redis_om::HashModel;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    middleware::auth::Auth,
    payload::chat::{ChatMessageContentPayload, ChatMessagePayload},
    state::{ApiDelta, AppState, ControlChunk},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompletionPayload {
    pub message: String,
    pub model: String,
    pub reasoning: Option<ReasoningEffort>,
    pub use_search: bool,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<ObjectId>,
    Auth(session): Auth,
    Json(payload): Json<PromptCompletionPayload>,
) -> impl IntoResponse {
    let user_id = ObjectId::from_str(&session.user_id).unwrap();
    let Some(model) = state
        .models()
        .free_models()
        .iter()
        .find(|model| model.identifier == payload.model)
        .cloned()
    else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Model does not exist." })),
        )
            .into_response();
    };

    let Ok(chat) = state
        .database()
        .chats
        .get(doc! { "_id": chat_id, "user_id": ObjectId::from_str(&session.user_id).unwrap() })
        .await
    else {
        tracing::error!("Failed to get chat.");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Some(chat) = chat else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat does not exist." })),
        )
            .into_response();
    };

    // MESSAGES

    let Ok(messages) = state
        .database()
        .messages
        .get_many_sorted(
            doc! { "chat_id": chat.id.unwrap() },
            doc! { "timestamp": -1 },
        )
        .await
    else {
        tracing::error!("Failed to get message.");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(mut messages) = messages
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
    else {
        tracing::error!("Failed to list message.");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    // FILES

    let files_chat_id = if messages.is_empty() {
        None
    } else {
        Some(chat.id.unwrap())
    };
    let Ok(files) = state
        .database()
        .uploads
        .get_many(doc! { "user_id": user_id, "chat_id": files_chat_id, "is_sent": false })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Ok(files) = files.try_collect::<Vec<UserUpload>>().await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    for file in files.iter() {
        if state
            .database()
            .uploads
            .update(
                file.id,
                doc! { "$set": { "chat_id": chat.id.unwrap(), "is_sent": true } },
            )
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    // API KEY

    let mut conn = state.redis();

    let api_key = if let Ok(cached_key) =
        UserApiKey::get(format!("openrouter-{}", session.user_id), &mut conn).await
    {
        Some(cached_key.key)
    } else {
        let key = state
            .database()
            .keys
            .get(doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap() })
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
        let (nonce, ciphertext) = api_key.split_once('.').unwrap();
        let nonce = hex::decode(nonce).unwrap();
        let ciphertext = hex::decode(ciphertext).unwrap();

        let key = Key::<Aes256Gcm>::from_slice(state.aes_key());
        let mut cipher = Aes256Gcm::new(&key);
        let plaintext = cipher
            .decrypt(GenericArray::from_slice(&nonce), ciphertext.as_ref())
            .unwrap();

        if model.base_url.starts_with("https://openrouter.ai") {
            OpenAIClient::new(String::from_utf8(plaintext).unwrap(), model.base_url)
        } else {
            todo!()
        }
    } else {
        state.openrouter().clone()
    };

    let (tx, rx) = flume::unbounded();

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
            let chat_name = OpenAIClient::new(
                env::var("CHUTES_KEY").unwrap(),
                "https://llm.chutes.ai/v1/completions".to_string(),
            )
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
                .database()
                .chats
                .update(chat.id.unwrap(), doc! { "$set": { "name": chat_name } })
                .await
                .unwrap();
        });
    }

    let user_message_id = state
        .database()
        .messages
        .create(user_message.clone())
        .await
        .unwrap();

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
                    .search(search_query.to_string())
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

        let message_for_assistant = if let Some(results) = search_results {
            let context: String = results
                .organic
                .iter()
                .take(10)
                .map(|result| {
                    format!(
                        " - Title: {};\nSnippet: {};\nSource: {};\n",
                        result.title, result.snippet, result.link
                    )
                })
                .collect();

            let mut user_message_content = user_message.content.clone();

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
            user_message.content.clone()
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
                                .bucket()
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
                                .bucket()
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
            chat_id: chat.id.unwrap(),
            timestamp: Utc::now(),
        };
        let task2_state = Arc::clone(&task_state);
        let task_message = assistant_message.clone();
        tokio::spawn(async move {
            task2_state
                .database()
                .messages
                .create(task_message)
                .await
                .unwrap();
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

    (
        StatusCode::OK,
        Json(json!({
          "stream_id": stream_id,
          "user_message": ChatMessagePayload {
            id: user_message_id,
            chat_id: user_message.chat_id,
            content,
            model: None,
            reasoning: None,
            role: user_message.role,
            timestamp: user_message.timestamp
          }
        })),
    )
        .into_response()
}
