use std::sync::Arc;

use axum::{
    Router,
    routing::{delete as method_delete, get, post},
};

use crate::state::AppState;

pub mod create;
pub mod delete;
pub mod list;
pub mod message;
pub mod messages;
pub mod rename;
pub mod share;
pub mod share_state;
pub mod state;
pub mod unshare;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chats", post(create::handler))
        .route("/chats", get(list::handler))
        .route("/chats/{chat_id}/message", post(message::handler))
        .route("/chats/{chat_id}/messages", get(messages::handler))
        .route("/chats/{chat_id}", method_delete(delete::handler))
        .route("/chats/{chat_id}/rename", post(rename::handler))
        .route("/chats/{chat_id}/share", post(share::handler))
        .route("/chats/{chat_id}/share", get(share_state::handler))
        .route(
            "/chats/{chat_id}/share/{share_id}",
            method_delete(unshare::handler),
        )
        .route("/chats/{chat_id}", get(state::handler))
}
