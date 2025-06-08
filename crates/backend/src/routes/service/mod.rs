use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub mod health;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health::handler))
}
