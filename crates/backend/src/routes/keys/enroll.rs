use std::{str::FromStr, sync::Arc};

use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{AeadMut, OsRng},
};
use axum::{Json, extract::State, response::IntoResponse};
use model::key::{ApiKey, UserApiKey};
use mongodb::bson::{doc, oid::ObjectId};
use redis_om::HashModel;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

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
) -> impl IntoResponse {
    if !VALID_INFERENCE_PROVIDERS.contains(&payload.provider.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid inference provider." })),
        )
            .into_response();
    }
    let user_id = ObjectId::from_str(&session.user_id).unwrap();
    let Ok(existing_key) = state
        .database()
        .keys
        .get(doc! { "user_id": user_id, "provider": payload.provider.clone() })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let key = Key::<Aes256Gcm>::from_slice(state.aes_key());
    let mut cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, payload.key.as_bytes()).unwrap();
    let key_str = format!("{}.{}", hex::encode(nonce), hex::encode(ciphertext));
    let key_id = if let Some(key) = existing_key {
        state
            .database()
            .keys
            .update(key.id.unwrap(), doc! { "key": key_str.clone() })
            .await
            .map(|_| key.id.unwrap())
    } else {
        state
            .database()
            .keys
            .create(ApiKey {
                id: None,
                key: key_str.clone(),
                provider: payload.provider.clone(),
                user_id,
            })
            .await
    };
    let Ok(key_id) = key_id else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to insert key into database." })),
        )
            .into_response();
    };

    let mut user_api_key = UserApiKey {
        id: format!("{}-{}", payload.provider, session.user_id),
        key_id: key_id.to_hex(),
        key: key_str,
    };

    let _ = user_api_key.save(&mut state.redis()).await;

    (StatusCode::OK, Json(json!({ "id": key_id.to_hex() }))).into_response()
}
