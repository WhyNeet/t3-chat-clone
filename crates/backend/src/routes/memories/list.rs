use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use futures::TryStreamExt;
use mongodb::bson::doc;
use reqwest::StatusCode;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::memories::MemoryPayload,
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
) -> Result<impl IntoResponse, ApplicationError> {
    let memories = state
        .storage()
        .database()
        .memories
        .get_many(doc! { "user_id": session.user_id })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let memories = memories
        .map_ok(|memory| MemoryPayload {
            id: memory.id.unwrap(),
            content: memory.content,
        })
        .try_collect::<Vec<MemoryPayload>>()
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow::anyhow!(e),
            )))
        })?;

    Ok((StatusCode::OK, Json(memories)).into_response())
}
