use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::state::AppState;

pub async fn handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let models = state.models().free_models();

    (StatusCode::OK, Json(models.to_vec()))
}
