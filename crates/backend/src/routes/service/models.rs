use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::state::AppState;

pub async fn handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let free_models = state.models().free_models();
    let paid_models = state.models().paid_models();

    (
        StatusCode::OK,
        Json(json!({ "free": free_models.to_vec(), "paid": paid_models.to_vec() })),
    )
}
