use std::{str::FromStr, sync::Arc, time::Duration};

use ai::openai::completions::OpenAIMessage;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::{StreamExt, TryStreamExt};
use model::message::{ChatMessage, Role};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    middleware::auth::Auth,
    state::{ApiDelta, AppState},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompletionPayload {
    pub message: String,
    pub model: String,
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

    let user_message = ChatMessage {
        id: None,
        content: payload.message.clone(),
        role: Role::User,
        chat_id: chat.id.unwrap(),
        index: messages.len() as u64,
    };

    state
        .database()
        .messages
        .create(user_message)
        .await
        .unwrap();

    messages.push(OpenAIMessage {
        role: "user".to_string(),
        content: payload.message,
    });

    let response_index = messages.len();
    let assistant_message_id = ObjectId::new();
    let task_state = Arc::clone(&state);
    tokio::spawn(async move {
        task_state
            .database()
            .messages
            .create(ChatMessage {
                id: Some(assistant_message_id),
                content: String::new(),
                role: Role::Assistant,
                chat_id: chat.id.unwrap(),
                index: response_index as u64,
            })
            .await
            .unwrap();
    });

    let client = state.openrouter().clone();
    let Ok(stream) = client.completion(payload.model, messages, Some(0.7)).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };
    tracing::debug!("Created stream.");
    let mut stream = Box::pin(stream);

    let stream_id = Uuid::new_v4();

    let (tx, rx) = flume::unbounded();
    let task_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut content = String::new();
        while let Ok(chunk) = stream.try_next().await {
            let Some(chunk) = chunk else { break };
            content.push_str(
                chunk
                    .choices
                    .get(0)
                    .unwrap()
                    .delta
                    .content
                    .as_ref()
                    .unwrap(),
            );
            tx.send_async(ApiDelta::Chunk(chunk)).await.unwrap();
        }
        tx.send(ApiDelta::Done).unwrap();
        task_state
            .database()
            .messages
            .update(
                assistant_message_id,
                doc! { "$set": { "content": content } },
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

    (StatusCode::OK, Json(json!({ "stream_id": stream_id }))).into_response()
}
