use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use serde_json::json;
use std::{str::FromStr, sync::Arc};

use crate::{middleware::auth::Auth, payload::chat::ChatMessagePayload, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ListChatMessagesPayload {
    pub start: usize,
    pub take: usize,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Query(payload): Query<ListChatMessagesPayload>,
) -> impl IntoResponse {
    let Ok(chat) = state
        .database()
        .chats
        .get(doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap(), "_id": chat_id })
        .await
    else {
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
                content: msg.content,
                timestamp: chat.timestamp,
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
