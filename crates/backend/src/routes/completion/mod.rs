use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub mod prompt;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route(
        "/completions/prompt/sse/{stream_id}",
        get(prompt::sse::handler),
    )
}
