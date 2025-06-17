use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use model::session::Session;
use redis_om::HashModel;
use reqwest::StatusCode;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, cache::CacheError},
    },
    middleware::auth::Auth,
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
) -> Result<impl IntoResponse, ApplicationError> {
    let mut conn = state.storage().cache().connection();
    Session::delete(session.session_id, &mut conn)
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::CacheError(CacheError::Unknown(e)))
        })?;

    Ok((StatusCode::OK).into_response())
}
