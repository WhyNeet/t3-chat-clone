use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use model::user::User;
use mongodb::error::{ErrorKind, WriteFailure};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    errors::{
        ApplicationError,
        crypto::CryptoError,
        storage::{StorageError, database::DatabaseError},
    },
    state::AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct AuthRegisterPayload {
    #[validate(email(message = "Invalid email."))]
    pub email: String,
    #[validate(length(min = 8, max = 72, message = "Password must be 8-72 chars long."))]
    pub password: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRegisterPayload>,
) -> Result<impl IntoResponse, ApplicationError> {
    if let Err(errors) = payload.validate() {
        return Err(ApplicationError::ValidationError(errors));
    }

    let hashed_password = state
        .crypto()
        .hash_password(payload.password.as_bytes())
        .map_err(|e| ApplicationError::CryptoError(CryptoError::Unknown(e)))?;

    let user = User {
        id: None,
        email: payload.email,
        password: hashed_password,
    };

    if let Err(e) = state.storage().database().users.create(user).await {
        let exists = match e
            .downcast_ref::<mongodb::error::Error>()
            .unwrap()
            .kind
            .as_ref()
        {
            ErrorKind::Write(err) => match err {
                WriteFailure::WriteError(err) => err.code == 11000,
                _ => false,
            },
            _ => false,
        };
        if exists {
            return Err(ApplicationError::StorageError(StorageError::DatabaseError(
                DatabaseError::UserAlreadyExists,
            )));
        }

        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::Unknown(e),
        )));
    }

    Ok((StatusCode::OK, Json(json!({}))).into_response())
}
