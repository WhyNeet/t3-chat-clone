use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Wrong password.")]
    WrongPassword,
    #[error("Unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for CryptoError {
    fn into_response(self) -> axum::response::Response {
        (
            match self {
                Self::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
                Self::WrongPassword => StatusCode::BAD_REQUEST,
            },
            Json(json!({ "error": self.to_string() })),
        )
            .into_response()
    }
}
