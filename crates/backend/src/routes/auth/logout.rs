use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use model::session::Session;
use redis_om::HashModel;
use reqwest::StatusCode;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(State(state): State<Arc<AppState>>, Auth(session): Auth) -> impl IntoResponse {
    let mut conn = state.redis();
    if Session::delete(session.id, &mut conn).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to delete session." })),
        )
            .into_response();
    };

    (StatusCode::OK).into_response()
}
