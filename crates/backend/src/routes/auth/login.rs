use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{payload::auth::UserPayload, state::AppState};

#[derive(Debug, Deserialize, Validate)]
pub struct AuthLoginPayload {
    #[validate(email(message = "Invalid email."))]
    pub email: String,
    #[validate(length(min = 8, max = 72, message = "Password must be 8-72 chars long."))]
    pub password: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthLoginPayload>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let Ok(user) = state.users().get_by(doc! { "email": payload.email }).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Some(user) = user else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "User does not exist." })),
        )
            .into_response();
    };

    let hash = PasswordHash::new(&user.password).unwrap();
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &hash)
        .is_err()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Wrong password." })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(UserPayload {
            id: user.id.unwrap(),
            email: user.email,
        }),
    )
        .into_response()
}
