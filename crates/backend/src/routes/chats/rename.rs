use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use mongodb::bson::{doc, oid::ObjectId};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ChatRenamePayload {
    pub name: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Json(payload): Json<ChatRenamePayload>,
) -> Result<impl IntoResponse, ApplicationError> {
    let name = payload.name.trim().to_string();
    let name = if name.is_empty() {
        "New Chat".to_string()
    } else {
        name
    };

    let chat = state
        .storage()
        .database()
        .chats
        .get_by_id(chat_id)
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let Some(chat) = chat else {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::ChatDoesNotExist,
        )));
    };

    if chat.user_id != session.user_id {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::ChatDoesNotBelongToUser,
        )));
    }

    state
        .storage()
        .database()
        .chats
        .update(chat.id.unwrap(), doc! { "$set": { "name": name } })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    Ok(StatusCode::OK.into_response())
}
