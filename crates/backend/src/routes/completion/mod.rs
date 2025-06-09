use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod prompt;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/completions/prompt/{chat_id}", post(prompt::handler))
        .route(
            "/completions/prompt/sse/{stream_id}",
            get(prompt::sse::handler),
        )
}
