use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use model::share::Share;
use mongodb::bson::oid::ObjectId;
use redis_om::HashModel;
use reqwest::StatusCode;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path((chat_id, share_id)): Path<(ObjectId, ObjectId)>,
) -> impl IntoResponse {
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

    let mut conn = state.redis();

    let Ok(share) = Share::get(chat_id.to_hex(), &mut conn).await else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat is not shared." })),
        )
            .into_response();
    };

    if share.share_id != share_id.to_hex() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat is not shared with this link." })),
        )
            .into_response();
    }

    if Share::delete(chat_id.to_hex(), &mut conn).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK).into_response()
}
