use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub mod create;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/chats/create", post(create::handler))
}
