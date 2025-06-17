use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("User already exists.")]
    UserAlreadyExists,
    #[error("User does not exist..")]
    UserDoesNotExist,
    #[error("Chat does not exist.")]
    ChatDoesNotExist,
    #[error("Chat does not belong to the user.")]
    ChatDoesNotBelongToUser,
    #[error("Unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        (
            match self {
                Self::UserAlreadyExists
                | Self::ChatDoesNotExist
                | Self::ChatDoesNotBelongToUser
                | Self::UserDoesNotExist => StatusCode::BAD_REQUEST,
                Self::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(json!({ "error": self.to_string() })),
        )
            .into_response()
    }
}
