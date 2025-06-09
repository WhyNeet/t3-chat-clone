use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub mod me;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/users/me", get(me::handler))
}
