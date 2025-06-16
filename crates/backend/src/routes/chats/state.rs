use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use model::share::Share;
use mongodb::bson::{doc, oid::ObjectId};
use redis_om::HashModel;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{middleware::auth::Auth, payload::chat::ChatPayload, state::AppState};

#[derive(Debug, Deserialize)]
pub struct GetChatStateQuery {
    share_id: Option<ObjectId>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Query(payload): Query<GetChatStateQuery>,
) -> impl IntoResponse {
    let is_shared = if let Some(share_id) = payload.share_id {
        let mut conn = state.redis();
        let Ok(share) = Share::get(chat_id.to_hex(), &mut conn).await else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        share.share_id == share_id.to_hex()
    } else {
        false
    };

    let chat = if is_shared {
        state.database().chats.get(doc! { "_id": chat_id }).await
    } else {
        state
            .database()
            .chats
            .get(doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap(), "_id": chat_id })
            .await
    };
    let Ok(chat) = chat else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Some(chat) = chat else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    (
        StatusCode::OK,
        Json(ChatPayload {
            id: chat.id.unwrap(),
            name: chat.name,
            user_id: chat.user_id,
            timestamp: chat.timestamp,
        }),
    )
        .into_response()
}
