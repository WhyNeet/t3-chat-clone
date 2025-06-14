use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use mongodb::bson::{Bson, doc, oid::ObjectId};
use reqwest::{StatusCode, header};
use serde_json::json;
use tokio_util::{compat::FuturesAsyncReadCompatExt, io::ReaderStream};

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path((chat_id, upload_id)): Path<(ObjectId, ObjectId)>,
) -> impl IntoResponse {
    let user_id = ObjectId::from_str(&session.user_id).unwrap();
    let Ok(upload) = state
        .database()
        .uploads
        .get(doc! { "chat_id": chat_id, "_id": upload_id, "user_id": user_id })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Some(upload) = upload else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Upload not found." })),
        )
            .into_response();
    };

    let Ok(stream) = state
        .bucket()
        .open_download_stream(Bson::ObjectId(upload.id))
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let stream = stream.compat();
    let stream = ReaderStream::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&upload.content_type).unwrap(),
    );

    (headers, Body::from_stream(stream)).into_response()
}

pub async fn no_chat_id_handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(upload_id): Path<ObjectId>,
) -> impl IntoResponse {
    let user_id = ObjectId::from_str(&session.user_id).unwrap();
    let Ok(upload) = state
        .database()
        .uploads
        .get(doc! { "chat_id": null, "_id": upload_id, "user_id": user_id })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Some(upload) = upload else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Upload not found." })),
        )
            .into_response();
    };

    let Ok(stream) = state
        .bucket()
        .open_download_stream(Bson::ObjectId(upload.id))
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let stream = stream.compat();
    let stream = ReaderStream::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&upload.content_type).unwrap(),
    );

    (headers, Body::from_stream(stream)).into_response()
}
