use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use mongodb::bson::{Bson, doc, oid::ObjectId};
use reqwest::StatusCode;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
) -> impl IntoResponse {
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

    if chat.user_id.to_hex() != session.user_id {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Chat does not belong to user." })),
        )
            .into_response();
    }

    if state
        .database()
        .chats
        .delete(chat.id.unwrap())
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    tokio::spawn(async move {
        let user_id = ObjectId::from_str(&session.user_id).unwrap();
        // delete messages
        let mut messages = state
            .database()
            .messages
            .get_many(doc! { "chat_id": chat_id })
            .await
            .unwrap()
            .into_stream();

        while let Ok(Some(message)) = messages.try_next().await {
            state
                .database()
                .messages
                .delete(message.id.unwrap())
                .await
                .unwrap();
        }

        // delete associated files
        let mut files = state
            .database()
            .uploads
            .get_many(doc! { "chat_id": chat_id, "user_id": user_id })
            .await
            .unwrap()
            .into_stream();

        while let Ok(Some(file)) = files.try_next().await {
            state.database().uploads.delete(file.id).await.unwrap();
            state
                .bucket()
                .delete(Bson::ObjectId(file.id))
                .await
                .unwrap();
        }
    });

    StatusCode::OK.into_response()
}
