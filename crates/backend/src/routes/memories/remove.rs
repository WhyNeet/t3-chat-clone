use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use mongodb::bson::oid::ObjectId;
use reqwest::StatusCode;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(memory_id): Path<ObjectId>,
) -> Result<impl IntoResponse, ApplicationError> {
    let memory = state
        .storage()
        .database()
        .memories
        .get_by_id(memory_id)
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let Some(memory) = memory else {
        return Err(ApplicationError::MemoryDoesNotExist);
    };

    if memory.user_id != session.user_id {
        return Err(ApplicationError::MemoryDoesNotBelongToUser);
    }

    state
        .storage()
        .database()
        .memories
        .delete(memory.id.unwrap())
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    Ok((StatusCode::OK).into_response())
}
