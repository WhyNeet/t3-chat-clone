use std::{str::FromStr, sync::Arc};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{middleware::auth::Auth, payload::auth::UserPayload, state::AppState};

pub async fn handler(State(state): State<Arc<AppState>>, Auth(session): Auth) -> impl IntoResponse {
    let Ok(user) = state
        .database()
        .users
        .get(ObjectId::from_str(&session.user_id).unwrap())
        .await
    else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    let Some(user) = user else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "User does not exist." })),
        )
            .into_response();
    };

    (
        StatusCode::OK,
        Json(UserPayload {
            id: user.id.unwrap(),
            email: user.email,
        }),
    )
        .into_response()
}
