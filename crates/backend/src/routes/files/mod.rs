use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod list;
pub mod stream;
pub mod upload;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/files/{chat_id}", post(upload::handler))
        .route("/files/{chat_id}", get(list::handler))
        .route("/files/{chat_id}/{upload_id}", get(stream::handler))
}
