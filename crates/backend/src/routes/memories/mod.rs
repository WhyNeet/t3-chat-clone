use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub mod list;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/memories", get(list::handler))
}
