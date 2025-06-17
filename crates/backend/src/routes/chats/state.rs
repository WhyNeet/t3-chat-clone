use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use model::share::Share;
use mongodb::bson::{doc, oid::ObjectId};
use redis_om::HashModel;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, cache::CacheError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::chat::ChatPayload,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct GetChatStateQuery {
    share_id: Option<ObjectId>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Query(payload): Query<GetChatStateQuery>,
) -> Result<impl IntoResponse, ApplicationError> {
    let is_shared = if let Some(share_id) = payload.share_id {
        let mut conn = state.storage().cache().connection();
        let share = Share::get(chat_id.to_hex(), &mut conn).await.map_err(|e| {
            ApplicationError::StorageError(StorageError::CacheError(CacheError::Unknown(e)))
        })?;

        share.share_id == share_id.to_hex()
    } else {
        false
    };

    let chat = if is_shared {
        state
            .storage()
            .database()
            .chats
            .get(doc! { "_id": chat_id })
            .await
    } else {
        state
            .storage()
            .database()
            .chats
            .get(doc! { "user_id": session.user_id, "_id": chat_id })
            .await
    }
    .map_err(|e| {
        ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
    })?;

    let Some(chat) = chat else {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::ChatDoesNotExist,
        )));
    };

    Ok((
        StatusCode::OK,
        Json(ChatPayload {
            id: chat.id.unwrap(),
            name: chat.name,
            user_id: chat.user_id,
            timestamp: chat.timestamp,
        }),
    )
        .into_response())
}
