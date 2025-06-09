use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod create;
pub mod list;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chats/create", post(create::handler))
        .route("/chats/list", get(list::handler))
}
