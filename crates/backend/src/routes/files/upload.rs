use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Multipart, Path, State},
    response::IntoResponse,
};
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use model::upload::UserUpload;
use mongodb::bson::oid::ObjectId;
use reqwest::StatusCode;
use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

use crate::{middleware::auth::Auth, payload::upload::UserUploadPayload, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    chat_id: Option<Path<ObjectId>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let chat_id = chat_id.map(|c| c.0);
    let user_id = ObjectId::from_str(&session.user_id).unwrap();
    let _ = if let Some(chat_id) = chat_id {
        let Ok(chat) = state.database().chats.get_by_id(chat_id).await else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };
        let Some(chat) = chat else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Chat does not exist." })),
            )
                .into_response();
        };

        if chat.user_id != user_id {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Chat does not belong to the user." })),
            )
                .into_response();
        }

        Some(chat)
    } else {
        None
    };

    let Ok(Some(file)) = multipart.next_field().await else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "File required." })),
        )
            .into_response();
    };

    let Some(content_type) = file.content_type() else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Missing file content type." })),
        )
            .into_response();
    };
    let content_type = content_type.to_string();

    if !["image/jpeg", "image/png", "application/pdf"].contains(&content_type.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid file content type." })),
        )
            .into_response();
    }

    let Ok(mut stream) = state.bucket().open_upload_stream("attachment").await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to initialize GridFS stream." })),
        )
            .into_response();
    };

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
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({ "error": "File too large." })),
                    )
                        .into_response();
                }
                // Write chunk into GridFS stream
                if let Err(e) = stream.write(&buffer[..n]).await {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            serde_json::json!({ "error": format!("Failed to write to GridFS: {}", e) }),
                        ),
                    )
                        .into_response();
                }
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": format!("Read error: {}", e) })),
                )
                    .into_response();
            }
        }
    }

    if let Err(e) = stream.close().await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::json!({ "error": format!("Failed to finalize GridFS upload: {}", e) }),
            ),
        )
            .into_response();
    }

    // create UserUpload

    let attachment_id = stream.id().as_object_id().unwrap();

    if state
        .database()
        .uploads
        .create(UserUpload {
            id: attachment_id,
            chat_id: chat_id,
            user_id,
            content_type: content_type.to_string(),
            is_sent: false,
        })
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create upload." })),
        )
            .into_response();
    }

    // created UserUpload

    (
        StatusCode::OK,
        Json(UserUploadPayload {
            id: attachment_id,
            chat_id,
            content_type,
            user_id,
        }),
    )
        .into_response()
}
