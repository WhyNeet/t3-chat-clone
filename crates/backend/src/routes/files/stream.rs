use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use mongodb::bson::{Bson, doc, oid::ObjectId};
use reqwest::header;
use tokio_util::{compat::FuturesAsyncReadCompatExt, io::ReaderStream};

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
    Path((chat_id, upload_id)): Path<(ObjectId, ObjectId)>,
) -> Result<impl IntoResponse, ApplicationError> {
    let upload = state
        .storage()
        .database()
        .uploads
        .get(doc! { "chat_id": chat_id, "_id": upload_id, "user_id": session.user_id })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;
    let Some(upload) = upload else {
        return Err(ApplicationError::UploadNotFound);
    };

    let stream = state
        .storage()
        .bucket()
        .gridfs()
        .open_download_stream(Bson::ObjectId(upload.id))
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    let stream = stream.compat();
    let stream = ReaderStream::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&upload.content_type).unwrap(),
    );

    Ok((headers, Body::from_stream(stream)).into_response())
}

pub async fn no_chat_id_handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(upload_id): Path<ObjectId>,
) -> Result<impl IntoResponse, ApplicationError> {
    let upload = state
        .storage()
        .database()
        .uploads
        .get(doc! { "chat_id": null, "_id": upload_id, "user_id": session.user_id })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;
    let Some(upload) = upload else {
        return Err(ApplicationError::UploadNotFound);
    };

    let stream = state
        .storage()
        .bucket()
        .gridfs()
        .open_download_stream(Bson::ObjectId(upload.id))
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    let stream = stream.compat();
    let stream = ReaderStream::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&upload.content_type).unwrap(),
    );

    Ok((headers, Body::from_stream(stream)).into_response())
}
