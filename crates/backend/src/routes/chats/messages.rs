use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::{StreamExt, TryStreamExt};
use model::{message::ChatMessageContent, share::Share};
use mongodb::bson::{doc, oid::ObjectId};
use redis_om::HashModel;
use serde::Deserialize;
use serde_json::json;
use std::{str::FromStr, sync::Arc};

use crate::{
    middleware::auth::Auth,
    payload::chat::{ChatMessageContentPayload, ChatMessagePayload},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListChatMessagesPayload {
    pub start: usize,
    pub take: usize,
    pub share_id: Option<ObjectId>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Query(payload): Query<ListChatMessagesPayload>,
) -> impl IntoResponse {
    let chat = if let Some(id) = payload.share_id {
        let mut conn = state.redis();
        let share = Share::get(chat_id.to_hex(), &mut conn).await.unwrap();
        if id.to_hex() != share.share_id {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Chat is not shared with this link." })),
            )
                .into_response();
        }
        state.database().chats.get(doc! { "_id": chat_id }).await
    } else {
        state
            .database()
            .chats
            .get(doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap(), "_id": chat_id })
            .await
    };

    let Ok(chat) = chat else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal error." })),
        )
            .into_response();
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
        .get_many_sorted(
            doc! { "chat_id": chat.id.unwrap() },
            doc! { "timestamp": -1 },
        )
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(messages) = messages
        .skip(payload.start)
        .take(payload.take)
        .map(|message| {
            message.map(|msg| ChatMessagePayload {
                id: msg.id.unwrap(),
                content: msg
                    .content
                    .into_iter()
                    .map(|message| match message {
                        ChatMessageContent::Text { value } => {
                            ChatMessageContentPayload::Text { value }
                        }
                        ChatMessageContent::Image { id } => ChatMessageContentPayload::Image { id },
                        ChatMessageContent::Pdf { id } => ChatMessageContentPayload::Pdf { id },
                    })
                    .collect(),
                model: msg.model,
                timestamp: chat.timestamp,
                reasoning: msg.reasoning,
                updated_memory: msg.updated_memory,
                chat_id: msg.chat_id,
                role: msg.role,
            })
        })
        .try_collect::<Vec<_>>()
        .await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal error." })),
        )
            .into_response();
    };

    (StatusCode::OK, Json(messages)).into_response()
}
