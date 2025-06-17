use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    Json,
    extract::{Multipart, Path, State},
    response::IntoResponse,
};
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use model::upload::UserUpload;
use mongodb::bson::oid::ObjectId;
use reqwest::StatusCode;
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

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
    chat_id: Option<Path<ObjectId>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApplicationError> {
    let chat_id = chat_id.map(|c| c.0);
    let _ = if let Some(chat_id) = chat_id {
        let chat = state
            .storage()
            .database()
            .chats
            .get_by_id(chat_id)
            .await
            .map_err(|e| {
                ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                    e,
                )))
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

        Some(chat)
    } else {
        None
    };

    let file = multipart
        .next_field()
        .await
        .map_err(|_| ApplicationError::FileRequired)?
        .ok_or(ApplicationError::FileRequired)?;

    let content_type = file
        .content_type()
        .ok_or(ApplicationError::NoFileContentType)?
        .to_string();

    if !["image/jpeg", "image/png", "application/pdf"].contains(&content_type.as_str()) {
        return Err(ApplicationError::InvalidFileContentType);
    }

    let mut stream = state
        .storage()
        .bucket()
        .gridfs()
        .open_upload_stream("attachment")
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    let file_stream = file.into_stream();

    let mut reader = StreamReader::new(
        file_stream
            .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))),
    );

    let mut size = 0;
    // Buffer to hold chunks
    let mut buffer = [0u8; 64 * 1024];

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => break, // End of stream
            Ok(n) => {
                size += n;
                if size > 1_000_000 {
                    stream.abort().await.unwrap();
                    return Err(ApplicationError::FileTooLarge);
                }
                // Write chunk into GridFS stream
                stream.write(&buffer[..n]).await.map_err(|e| {
                    ApplicationError::StorageError(StorageError::DatabaseError(
                        DatabaseError::Unknown(anyhow!(e)),
                    ))
                })?;
            }
            Err(e) => {
                return Err(ApplicationError::StorageError(StorageError::DatabaseError(
                    DatabaseError::Unknown(anyhow!(e)),
                )));
            }
        }
    }

    stream.close().await.map_err(|e| {
        ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
            anyhow!(e),
        )))
    })?;

    // create UserUpload

    let attachment_id = stream.id().as_object_id().unwrap();

    state
        .storage()
        .database()
        .uploads
        .create(UserUpload {
            id: attachment_id,
            chat_id: chat_id,
            user_id: session.user_id,
            content_type: content_type.to_string(),
            is_sent: false,
        })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    // created UserUpload

    Ok((
        StatusCode::OK,
        Json(UserUploadPayload {
            id: attachment_id,
            chat_id,
            content_type,
            user_id: session.user_id,
        }),
    )
        .into_response())
}
