use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use model::key::UserApiKey;
use mongodb::bson::oid::ObjectId;
use redis_om::HashModel;
use reqwest::StatusCode;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(key_id): Path<ObjectId>,
) -> impl IntoResponse {
    let Ok(key) = state.database().keys.get_by_id(key_id).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Some(key) = key else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Key does not exist." })),
        )
            .into_response();
    };

    if key.user_id.to_hex() != session.user_id {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Key does not belong to the user." })),
        )
            .into_response();
    }

    if state.database().keys.delete(key.id.unwrap()).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let mut conn = state.redis();
    let _ = UserApiKey::delete(
        format!("{}-{}", key.provider, key.user_id.to_hex()),
        &mut conn,
    )
    .await;

    StatusCode::OK.into_response()
}
