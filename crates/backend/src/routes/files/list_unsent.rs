use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use reqwest::StatusCode;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::upload::UserUploadPayload,
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
) -> Result<impl IntoResponse, ApplicationError> {
    let uploads = state
        .storage()
        .database()
        .uploads
        .get_many(doc! { "chat_id": chat_id, "user_id": session.user_id, "is_sent": false })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let uploads = uploads
        .map_ok(|upload| UserUploadPayload {
            id: upload.id,
            chat_id: upload.chat_id,
            user_id: upload.user_id,
            content_type: upload.content_type,
        })
        .try_collect::<Vec<UserUploadPayload>>()
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    Ok((StatusCode::OK, Json(uploads)).into_response())
}

pub async fn no_chat_id_handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
) -> Result<impl IntoResponse, ApplicationError> {
    let uploads = state
        .storage()
        .database()
        .uploads
        .get_many(doc! { "chat_id": null, "user_id": session.user_id, "is_sent": false })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let uploads = uploads
        .map_ok(|upload| UserUploadPayload {
            id: upload.id,
            chat_id: upload.chat_id,
            user_id: upload.user_id,
            content_type: upload.content_type,
        })
        .try_collect::<Vec<UserUploadPayload>>()
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    Ok((StatusCode::OK, Json(uploads)).into_response())
}
