use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use cookie::time::Duration;
use hmac::Mac;
use model::session::Session;
use mongodb::bson::doc;
use redis_om::HashModel;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    payload::auth::UserPayload,
    routes::auth::{HmacSha256, SESSION_EXPIRATION, SESSION_ID_COOKIE_NAME},
    state::AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct AuthLoginPayload {
    #[validate(email(message = "Invalid email."))]
    pub email: String,
    #[validate(length(min = 8, max = 72, message = "Password must be 8-72 chars long."))]
    pub password: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(payload): Json<AuthLoginPayload>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let Ok(user) = state
        .database()
        .users
        .get(doc! { "email": payload.email })
        .await
    else {
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

    let session_id = Uuid::new_v4();
    let mut session = Session {
        id: session_id.to_string(),
        user_id: user.id.unwrap().to_hex(),
    };
    let mut conn = state.redis();
    if let Err(e) = session.save(&mut conn).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response();
    } else {
        session.expire(SESSION_EXPIRATION, &mut conn).await.unwrap();
    }

    let mut mac = HmacSha256::new_from_slice(state.hmac_key()).unwrap();
    mac.update(session_id.to_string().as_bytes());

    let signature = mac.finalize().into_bytes();
    let signature = hex::encode(signature);
    let cookie = Cookie::build((SESSION_ID_COOKIE_NAME, format!("{session_id}.{signature}")))
        .path("/")
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(SESSION_EXPIRATION as i64))
        .build();

    (
        StatusCode::OK,
        jar.add(cookie),
        Json(UserPayload {
            id: user.id.unwrap(),
            email: user.email,
        }),
    )
        .into_response()
}
