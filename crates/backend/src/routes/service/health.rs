use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn handler() -> impl IntoResponse {
    Json(json!({ "status": "running" }))
}
