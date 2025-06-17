use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use model::share::Share;
use mongodb::bson::oid::ObjectId;
use redis_om::HashModel;
use reqwest::StatusCode;
use serde_json::json;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, cache::CacheError, database::DatabaseError},
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

    let mut conn = state.storage().cache().connection();

    let share_id = ObjectId::new();
    let mut share = Share {
        id: chat_id.to_hex(),
        share_id: share_id.to_hex(),
    };

    share.save(&mut conn).await.map_err(|e| {
        ApplicationError::StorageError(StorageError::CacheError(CacheError::Unknown(e)))
    })?;

    Ok((StatusCode::OK, Json(json!({ "id": share_id.to_hex() }))).into_response())
}
