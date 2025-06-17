use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::auth::UserPayload,
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
) -> Result<impl IntoResponse, ApplicationError> {
    let user = state
        .storage()
        .database()
        .users
        .get_by_id(session.user_id)
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let Some(user) = user else {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::UserDoesNotExist,
        )));
    };

    Ok((
        StatusCode::OK,
        Json(UserPayload {
            id: user.id.unwrap(),
            email: user.email,
        }),
    )
        .into_response())
}
