use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub mod health;
pub mod models;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health::handler))
        .route("/models", get(models::handler))
}
