use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use mongodb::bson::{Bson, doc, oid::ObjectId};
use reqwest::StatusCode;
use serde_json::json;

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

    if state
        .bucket()
        .delete(Bson::ObjectId(upload.id))
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to delete upload." })),
        )
            .into_response();
    }

    if state.database().uploads.delete(upload.id).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to remove upload." })),
        )
            .into_response();
    }

    (StatusCode::OK).into_response()
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

    if state
        .bucket()
        .delete(Bson::ObjectId(upload.id))
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to delete upload." })),
        )
            .into_response();
    }

    if state.database().uploads.delete(upload.id).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to remove upload." })),
        )
            .into_response();
    }

    (StatusCode::OK).into_response()
}
