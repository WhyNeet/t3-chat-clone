use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use model::upload::UserUpload;
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
        // delete associated files
        let files = state
            .database()
            .uploads
            .get_many(doc! { "chat_id": chat_id, "user_id": ObjectId::from_str(&session.user_id).unwrap() })
            .await
            .unwrap()
            .try_collect::<Vec<UserUpload>>()
            .await
            .unwrap();

        for file in files {
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
