use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use reqwest::StatusCode;

use crate::{middleware::auth::Auth, payload::upload::UserUploadPayload, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
) -> impl IntoResponse {
    let user_id = ObjectId::from_str(&session.user_id).unwrap();
    let Ok(uploads) = state
        .database()
        .uploads
        .get_many(doc! { "chat_id": chat_id, "user_id": user_id })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Ok(uploads) = uploads
        .map_ok(|upload| UserUploadPayload {
            id: upload.id,
            chat_id: upload.chat_id,
            user_id: upload.user_id,
            content_type: upload.content_type,
        })
        .try_collect::<Vec<UserUploadPayload>>()
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (StatusCode::OK, Json(uploads)).into_response()
}
