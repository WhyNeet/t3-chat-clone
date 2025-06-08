use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub mod prompt;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/completion/prompt", post(prompt::handler))
}
