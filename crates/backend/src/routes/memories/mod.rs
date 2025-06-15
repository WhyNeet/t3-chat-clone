use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get},
};

use crate::state::AppState;

pub mod list;
pub mod remove;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/memories", get(list::handler))
        .route("/memories/{memory_id}", delete(remove::handler))
}
