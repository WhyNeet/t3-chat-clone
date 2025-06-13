use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use mongodb::bson::{doc, oid::ObjectId};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ChatRenamePayload {
    pub name: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Json(payload): Json<ChatRenamePayload>,
) -> impl IntoResponse {
    let name = payload.name.trim().to_string();
    let name = if name.is_empty() {
        "New Chat".to_string()
    } else {
        name
    };

    let Ok(chat) = state.database().chats.get_by_id(chat_id).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Some(chat) = chat else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat does not exist." })),
        )
            .into_response();
    };

    if chat.user_id.to_hex() != session.user_id {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat does not belong to user." })),
        )
            .into_response();
    }

    if state
        .database()
        .chats
        .update(chat.id.unwrap(), doc! { "$set": { "name": name } })
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    StatusCode::OK.into_response()
}
