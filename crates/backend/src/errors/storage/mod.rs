pub mod cache;
pub mod database;
use axum::response::IntoResponse;
use thiserror::Error;

use database::DatabaseError;

use crate::errors::storage::cache::CacheError;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("{0}")]
    DatabaseError(DatabaseError),
    #[error("Cache error: {0}")]
    CacheError(CacheError),
}

impl IntoResponse for StorageError {
    fn into_response(self) -> axum::response::Response {
        match self {
            StorageError::DatabaseError(error) => error.into_response(),
            StorageError::CacheError(error) => error.into_response(),
        }
    }
}
