use std::{str::FromStr, sync::Arc};

use axum::{Json, extract::State, response::IntoResponse};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use reqwest::StatusCode;

use crate::{middleware::auth::Auth, payload::memories::MemoryPayload, state::AppState};

pub async fn handler(State(state): State<Arc<AppState>>, Auth(session): Auth) -> impl IntoResponse {
    let Ok(memories) = state
        .database()
        .memories
        .get_many(doc! { "user_id": ObjectId::from_str(&session.user_id).unwrap() })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(memories) = memories
        .map_ok(|memory| MemoryPayload {
            id: memory.id.unwrap(),
            content: memory.content,
        })
        .try_collect::<Vec<MemoryPayload>>()
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (StatusCode::OK, Json(memories)).into_response()
}
