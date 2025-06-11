use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use model::user::User;
use mongodb::error::{ErrorKind, WriteFailure};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::state::AppState;

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
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let hashed_password = {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .map(|h| h.to_string())
    };
    let Ok(hashed_password) = hashed_password else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let user = User {
        id: None,
        email: payload.email,
        password: hashed_password,
    };

    if let Err(e) = state.database().users.create(user).await {
        let error_string = e.to_string();
        let exists = match e.downcast::<mongodb::error::Error>().unwrap().kind.as_ref() {
            ErrorKind::Write(err) => match err {
                WriteFailure::WriteError(err) => err.code == 11000,
                _ => false,
            },
            _ => false,
        };
        return (
          if exists { StatusCode::BAD_REQUEST } else { StatusCode::INTERNAL_SERVER_ERROR },
            Json(json!({ "error": if exists { "User with this email already exists." } else { &error_string }  })),
        )
            .into_response();
    }

    (StatusCode::OK, Json(json!({}))).into_response()
}
