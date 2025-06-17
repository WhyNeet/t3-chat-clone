use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use model::chat::Chat;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::chat::ChatPayload,
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
) -> Result<impl IntoResponse, ApplicationError> {
    let chat = Chat {
        id: None,
        name: None,
        user_id: session.user_id,
        timestamp: Utc::now(),
    };

    let id = state
        .storage()
        .database()
        .chats
        .create(chat.clone())
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    Ok((
        StatusCode::OK,
        Json(ChatPayload {
            id,
            name: chat.name,
            user_id: chat.user_id,
            timestamp: chat.timestamp,
        }),
    )
        .into_response())
}
