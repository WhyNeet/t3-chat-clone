use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::state::AppState;

pub mod list;
pub mod remove;
pub mod stream;
pub mod upload;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/files/{chat_id}", post(upload::handler))
        .route("/files/nochat", post(upload::handler))
        .route("/files/{chat_id}", get(list::handler))
        .route("/files/nochat", get(list::no_chat_id_handler))
        .route("/files/{chat_id}/{upload_id}", get(stream::handler))
        .route("/files/nochat/{upload_id}", get(stream::no_chat_id_handler))
        .route("/files/{chat_id}/{upload_id}", delete(remove::handler))
        .route(
            "/files/nochat/{upload_id}",
            delete(remove::no_chat_id_handler),
        )
}
