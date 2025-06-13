use std::{str::FromStr, sync::Arc};

use axum::{Json, extract::State, response::IntoResponse};
use futures::TryStreamExt;
use model::key::UserApiKey;
use mongodb::bson::{doc, oid::ObjectId};
use redis_om::HashModel;
use reqwest::StatusCode;
use serde_json::json;

use crate::{middleware::auth::Auth, state::AppState};

pub async fn handler(State(state): State<Arc<AppState>>, Auth(session): Auth) -> impl IntoResponse {
    let mut conn = state.redis();
    let keys = if let Ok(key) =
        UserApiKey::get(format!("openrouter-{}", session.user_id), &mut conn).await
    {
        vec![(key.id, key.key, "openrouter".to_string())]
    } else {
        let keys = state
            .database()
            .keys
            .get_many(doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap() })
            .await
            .unwrap()
            .map_ok(|key| (key.id.unwrap().to_hex(), key.key, key.provider))
            .try_collect::<Vec<(String, String, String)>>()
            .await
            .unwrap();
        for (key_id, key, provider) in keys.iter().cloned() {
            let mut key = UserApiKey {
                id: format!("{provider}-{}", session.user_id),
                key_id,
                key,
            };

            let _ = key.save(&mut conn).await;
        }

        keys
    };

    let keys_json: Vec<serde_json::Value> = keys
        .into_iter()
        .map(|(key_id, _, key_provider)| json!({ "id": key_id, "provider": key_provider }))
        .collect();
    (StatusCode::OK, Json(json!({ "keys": keys_json })))
}
