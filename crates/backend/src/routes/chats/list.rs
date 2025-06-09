use std::{str::FromStr, sync::Arc};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;

use crate::{middleware::auth::Auth, payload::chat::ChatPayload, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ListChatsPayload {
    pub start: usize,
    pub take: usize,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Json(payload): Json<ListChatsPayload>,
) -> impl IntoResponse {
    let Ok(chats) = state
        .database()
        .chats
        .get_many_sorted(
            doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap() },
            doc! { "timestamp": -1 },
        )
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(chats) = chats
        .skip(payload.start)
        .take(payload.take)
        .map(|chat| {
            chat.map(|chat| ChatPayload {
                id: chat.id.unwrap(),
                name: chat.name,
                timestamp: chat.timestamp,
                user_id: chat.user_id,
            })
        })
        .try_collect::<Vec<_>>()
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (StatusCode::OK, Json(chats)).into_response()
}
