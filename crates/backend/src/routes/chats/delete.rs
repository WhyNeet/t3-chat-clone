use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use mongodb::bson::{Bson, doc, oid::ObjectId};
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
    Path(chat_id): Path<ObjectId>,
) -> Result<impl IntoResponse, ApplicationError> {
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
        .delete(chat.id.unwrap())
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    tokio::spawn(async move {
        // delete messages
        let mut messages = state
            .storage()
            .database()
            .messages
            .get_many(doc! { "chat_id": chat_id })
            .await
            .unwrap()
            .into_stream();

        while let Ok(Some(message)) = messages.try_next().await {
            state
                .storage()
                .database()
                .messages
                .delete(message.id.unwrap())
                .await
                .unwrap();
        }

        // delete associated files
        let mut files = state
            .storage()
            .database()
            .uploads
            .get_many(doc! { "chat_id": chat_id, "user_id": session.user_id })
            .await
            .unwrap()
            .into_stream();

        while let Ok(Some(file)) = files.try_next().await {
            state
                .storage()
                .database()
                .uploads
                .delete(file.id)
                .await
                .unwrap();
            state
                .storage()
                .bucket()
                .gridfs()
                .delete(Bson::ObjectId(file.id))
                .await
                .unwrap();
        }
    });

    Ok(StatusCode::OK.into_response())
}
