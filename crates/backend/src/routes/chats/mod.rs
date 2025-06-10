use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod create;
pub mod list;
pub mod message;
pub mod messages;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chats", post(create::handler))
        .route("/chats", get(list::handler))
        .route("/chats/{chat_id}/message", post(message::handler))
        .route("/chats/{chat_id}/messages", get(messages::handler))
}
