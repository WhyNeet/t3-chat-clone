use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use cookie::time::Duration;
use model::session::Session;
use mongodb::bson::doc;
use redis_om::HashModel;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::{
        ApplicationError,
        crypto::CryptoError,
        storage::{StorageError, cache::CacheError, database::DatabaseError},
    },
    payload::auth::UserPayload,
    routes::auth::{SESSION_EXPIRATION, SESSION_ID_COOKIE_NAME},
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
) -> Result<impl IntoResponse, ApplicationError> {
    if let Err(errors) = payload.validate() {
        return Ok((StatusCode::BAD_REQUEST, Json(errors)).into_response());
    }

    let Ok(user) = state
        .storage()
        .database()
        .users
        .get(doc! { "email": payload.email })
        .await
    else {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };

    let Some(user) = user else {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::UserAlreadyExists,
        )));
    };

    if !state
        .crypto()
        .verify_password(&user.password, payload.password.as_bytes())
        .map_err(|e| ApplicationError::CryptoError(CryptoError::Unknown(e)))?
    {
        return Err(ApplicationError::CryptoError(CryptoError::WrongPassword));
    }

    let session_id = Uuid::new_v4().to_string();
    let mut session = Session {
        id: session_id.clone(),
        user_id: user.id.unwrap().to_hex(),
    };
    let mut conn = state.storage().cache().connection();
    session.save(&mut conn).await.map_err(|e| {
        ApplicationError::StorageError(StorageError::CacheError(CacheError::Unknown(e)))
    })?;

    session
        .expire(SESSION_EXPIRATION, &mut conn)
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::CacheError(CacheError::Unknown(e)))
        })?;

    let signature = state
        .crypto()
        .sign_session(session_id.as_bytes())
        .map_err(|e| ApplicationError::CryptoError(CryptoError::Unknown(e)))?;

    let cookie = Cookie::build((SESSION_ID_COOKIE_NAME, format!("{session_id}.{signature}")))
        .path("/")
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(SESSION_EXPIRATION as i64))
        .build();

    Ok((
        StatusCode::OK,
        jar.add(cookie),
        Json(UserPayload {
            id: user.id.unwrap(),
            email: user.email,
        }),
    )
        .into_response())
}
