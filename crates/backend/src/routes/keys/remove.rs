use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use model::key::UserApiKey;
use mongodb::bson::oid::ObjectId;
use redis_om::HashModel;
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
    Path(key_id): Path<ObjectId>,
) -> Result<impl IntoResponse, ApplicationError> {
    let key = state
        .storage()
        .database()
        .keys
        .get_by_id(key_id)
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let Some(key) = key else {
        return Err(ApplicationError::KeyDoesNotExist);
    };

    if key.user_id != session.user_id {
        return Err(ApplicationError::KeyDoesNotBelongToUser);
    }

    state
        .storage()
        .database()
        .keys
        .delete(key.id.unwrap())
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let mut conn = state.storage().cache().connection();
    let _ = UserApiKey::delete(
        format!("{}-{}", key.provider, key.user_id.to_hex()),
        &mut conn,
    )
    .await;

    Ok(StatusCode::OK.into_response())
}
