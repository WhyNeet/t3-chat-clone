use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub mod upload;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/files/{chat_id}", post(upload::handler))
}
