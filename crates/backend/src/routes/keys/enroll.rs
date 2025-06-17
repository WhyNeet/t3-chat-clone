use axum::{Json, extract::State, response::IntoResponse};
use model::key::{ApiKey, UserApiKey};
use mongodb::bson::doc;
use redis_om::HashModel;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::{
    errors::{
        ApplicationError,
        crypto::CryptoError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    state::AppState,
};

const VALID_INFERENCE_PROVIDERS: &[&'static str] = &["openrouter"];

#[derive(Debug, Deserialize)]
pub struct KeyEnrollPayload {
    pub key: String,
    pub provider: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Json(payload): Json<KeyEnrollPayload>,
) -> Result<impl IntoResponse, ApplicationError> {
    if !VALID_INFERENCE_PROVIDERS.contains(&payload.provider.as_str()) {
        return Err(ApplicationError::InvalidInferenceProvider);
    }
    let existing_key = state
        .storage()
        .database()
        .keys
        .get(doc! { "user_id": session.user_id, "provider": payload.provider.clone() })
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let key_str = state
        .crypto()
        .encrypt_key(payload.key.as_bytes())
        .map_err(|e| ApplicationError::CryptoError(CryptoError::Unknown(e)))?;

    let key_id = if let Some(key) = existing_key {
        state
            .storage()
            .database()
            .keys
            .update(key.id.unwrap(), doc! { "key": key_str.clone() })
            .await
            .map(|_| key.id.unwrap())
    } else {
        state
            .storage()
            .database()
            .keys
            .create(ApiKey {
                id: None,
                key: key_str.clone(),
                provider: payload.provider.clone(),
                user_id: session.user_id,
            })
            .await
    }
    .map_err(|e| {
        ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
    })?;

    let mut user_api_key = UserApiKey {
        id: format!("{}-{}", payload.provider, session.user_id),
        key_id: key_id.to_hex(),
        key: key_str,
    };

    let _ = user_api_key
        .save(&mut state.storage().cache().connection())
        .await;

    Ok((StatusCode::OK, Json(json!({ "id": key_id.to_hex() }))).into_response())
}
