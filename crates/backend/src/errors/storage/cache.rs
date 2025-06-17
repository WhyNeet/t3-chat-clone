use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Unknown error: {0}")]
    Unknown(#[from] redis_om::RedisError),
}

impl IntoResponse for CacheError {
    fn into_response(self) -> axum::response::Response {
        (
            match self {
                Self::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(json!({ "error": self.to_string() })),
        )
            .into_response()
    }
}
