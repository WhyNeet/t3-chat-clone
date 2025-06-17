use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::doc;
use serde::Deserialize;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::chat::ChatPayload,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListChatsPayload {
    pub start: usize,
    pub take: usize,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Query(payload): Query<ListChatsPayload>,
) -> Result<impl IntoResponse, ApplicationError> {
    let chats = state
        .storage()
        .database()
        .chats
        .get_many_sorted(doc! { "user_id": session.user_id }, doc! { "timestamp": 1 })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let chats = chats
        .skip(payload.start)
        .take(payload.take)
        .map(|chat| {
            chat.map(|chat| ChatPayload {
                id: chat.id.unwrap(),
                name: chat.name,
                timestamp: chat.timestamp,
                user_id: chat.user_id,
            })
        })
        .try_collect::<Vec<_>>()
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow::anyhow!(e),
            )))
        })?;

    Ok((StatusCode::OK, Json(chats)).into_response())
}
