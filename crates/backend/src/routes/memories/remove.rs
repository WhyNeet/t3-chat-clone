use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use mongodb::bson::oid::ObjectId;
use reqwest::StatusCode;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(memory_id): Path<ObjectId>,
) -> impl IntoResponse {
    let Ok(memory) = state.database().memories.get_by_id(memory_id).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Some(memory) = memory else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Memory does not exist." })),
        )
            .into_response();
    };
    if memory.user_id.to_hex() != session.user_id {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Memory does not belong to the user." })),
        )
            .into_response();
    }

    if state
        .database()
        .memories
        .delete(memory.id.unwrap())
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK).into_response()
}
