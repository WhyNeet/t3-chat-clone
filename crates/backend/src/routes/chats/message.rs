use std::{str::FromStr, sync::Arc, time::Duration};

use ai::openai::completions::{OpenAICompletionDelta, OpenAIMessage, ReasoningEffort};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use model::message::{ChatMessage, Role};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    middleware::auth::Auth,
    payload::chat::ChatMessagePayload,
    state::{ApiDelta, AppState},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompletionPayload {
    pub message: String,
    pub model: String,
    pub reasoning: Option<ReasoningEffort>,
    pub use_search: bool,
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<ObjectId>,
    Auth(session): Auth,
    Json(payload): Json<PromptCompletionPayload>,
) -> impl IntoResponse {
    let Ok(chat) = state
        .database()
        .chats
        .get(doc! { "_id": chat_id, "user_id": ObjectId::from_str(&session.user_id).unwrap() })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Some(chat) = chat else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat does not exist." })),
        )
            .into_response();
    };

    let Ok(messages) = state
        .database()
        .messages
        .get_many_sorted(doc! { "chat_id": chat.id.unwrap() }, doc! { "index": 1 })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(mut messages) = messages
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|msg| {
            msg.map(|msg| OpenAIMessage {
                content: msg.content,
                role: msg.role.to_string(),
            })
        })
        .collect::<Result<Vec<_>, mongodb::error::Error>>()
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let search_results = if payload.use_search {
        let search_query = payload.message.trim();
        if payload.message.is_empty() || payload.message.len() > 400 {
            None
        } else {
            state.search().search(search_query.to_string()).await.ok()
        }
    } else {
        None
    };

    let user_message = ChatMessage {
        id: None,
        content: if let Some(results) = search_results {
            let context: String = results
                .organic
                .iter()
                .take(10)
                .map(|result| {
                    format!(
                        "- {}: {}\n  Source: {}\n",
                        result.title, result.snippet, result.link
                    )
                })
                .collect();

            format!(
                "Use the following search results to answer the query. If information is insufficient, state that.\n\n\
                    Search Results:\n{context}\n\n\
                    Query: {}\n\nAnswer:",
                payload.message
            )
        } else {
            payload.message.clone()
        },
        reasoning: None,
        role: Role::User,
        chat_id: chat.id.unwrap(),
        timestamp: Utc::now(),
    };

    let user_message_id = state
        .database()
        .messages
        .create(user_message.clone())
        .await
        .unwrap();

    messages.push(OpenAIMessage {
        role: "user".to_string(),
        content: payload.message,
    });

    let assistant_message_id = ObjectId::new();
    let assistant_message = ChatMessage {
        id: Some(assistant_message_id),
        content: String::new(),
        role: Role::Assistant,
        reasoning: None,
        chat_id: chat.id.unwrap(),
        timestamp: Utc::now(),
    };
    let task_state = Arc::clone(&state);
    let task_message = assistant_message.clone();
    tokio::spawn(async move {
        task_state
            .database()
            .messages
            .create(task_message)
            .await
            .unwrap();
    });

    let client = state.openrouter().clone();
    let Ok(stream) = client
        .completion(payload.model, messages, Some(0.7), payload.reasoning)
        .await
    else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };
    tracing::debug!("Created stream.");
    let mut stream = Box::pin(stream);

    let stream_id = Uuid::new_v4();

    let (tx, rx) = flume::unbounded();
    let task_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut reasoning: Option<String> = None;
        let mut content = String::new();

        let mut reasoning_acc: Option<String> = None;
        let mut content_acc = String::new();
        let mut iteration_start = Utc::now().timestamp_millis();
        while let Ok(chunk) = stream.try_next().await {
            let Some(chunk) = chunk else { break };
            let delta = &chunk.choices.get(0).unwrap().delta;
            let reasoning_content = delta.reasoning.as_ref();
            let delta_content = delta.content.as_ref().unwrap();
            if let Some(reasoning_content) = reasoning_content {
                if let Some(ref mut reasoning) = reasoning {
                    reasoning.push_str(reasoning_content);
                } else {
                    reasoning = Some(reasoning_content.to_string())
                }

                if reasoning_acc.is_none() {
                    reasoning_acc = Some(reasoning_content.clone());
                } else {
                    reasoning_acc.as_mut().unwrap().push_str(reasoning_content);
                }
            }
            content.push_str(delta_content);
            content_acc.push_str(delta_content);
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
        tx.send(ApiDelta::Done(ChatMessage {
            content: content.clone(),
            reasoning: reasoning.clone(),
            ..assistant_message
        }))
        .unwrap();
        task_state
            .database()
            .messages
            .update(
                assistant_message_id,
                doc! { "$set": { "content": content, "reasoning": reasoning } },
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

    (
        StatusCode::OK,
        Json(json!({
          "stream_id": stream_id,
          "user_message": ChatMessagePayload {
            id: user_message_id,
            chat_id: user_message.chat_id,
            content: user_message.content,
            reasoning: None,
            role: user_message.role,
            timestamp: user_message.timestamp
          }
        })),
    )
        .into_response()
}
