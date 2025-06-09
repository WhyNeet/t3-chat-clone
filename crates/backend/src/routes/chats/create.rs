use std::{str::FromStr, sync::Arc};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use model::chat::Chat;
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{middleware::auth::Auth, payload::chat::ChatPayload, state::AppState};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChatCreatePayload {

// }

pub async fn handler(State(state): State<Arc<AppState>>, Auth(session): Auth) -> impl IntoResponse {
    let chat = Chat {
        id: None,
        name: None,
        user_id: ObjectId::from_str(&session.user_id).unwrap(),
        timestamp: Utc::now(),
    };

    let Ok(id) = state.database().chats.create(chat.clone()).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (
        StatusCode::OK,
        Json(ChatPayload {
            id: id.to_hex(),
            name: chat.name,
            user_id: chat.user_id.to_hex(),
            timestamp: chat.timestamp,
        }),
    )
        .into_response()
}
