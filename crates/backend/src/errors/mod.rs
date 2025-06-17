pub mod crypto;
pub mod storage;

use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

use storage::StorageError;

use crate::errors::crypto::CryptoError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{0}")]
    StorageError(StorageError),
    #[error("Crypto error: {0}")]
    CryptoError(CryptoError),
    #[error("Validation error: {0}")]
    ValidationError(validator::ValidationErrors),

    #[error("Invalid model identifier.")]
    InvalidModelIdentifier,
    #[error("Invalid chat share link.")]
    InvalidShareLink,

    #[error("Upload not found.")]
    UploadNotFound,

    #[error("File required.")]
    FileRequired,
    #[error("Missing file content type.")]
    NoFileContentType,
    #[error("Invalid file content type.")]
    InvalidFileContentType,
    #[error("File is too large.")]
    FileTooLarge,

    #[error("Invalid inference provider.")]
    InvalidInferenceProvider,

    #[error("Key does not exist.")]
    KeyDoesNotExist,
    #[error("Key does not belong to the user.")]
    KeyDoesNotBelongToUser,

    #[error("Memory does not exist.")]
    MemoryDoesNotExist,
    #[error("Memory does not belong to the user.")]
    MemoryDoesNotBelongToUser,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::StorageError(error) => error.into_response(),
            Self::CryptoError(error) => error.into_response(),
            Self::ValidationError(errors) => {
                (StatusCode::BAD_REQUEST, Json(json!({ "error": errors }))).into_response()
            }
            Self::InvalidModelIdentifier
            | Self::InvalidShareLink
            | Self::UploadNotFound
            | Self::FileRequired
            | Self::NoFileContentType
            | Self::InvalidFileContentType
            | Self::FileTooLarge
            | Self::InvalidInferenceProvider
            | Self::KeyDoesNotExist
            | Self::KeyDoesNotBelongToUser
            | Self::MemoryDoesNotExist
            | Self::MemoryDoesNotBelongToUser => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": self.to_string() })),
            )
                .into_response(),
        }
    }
}
