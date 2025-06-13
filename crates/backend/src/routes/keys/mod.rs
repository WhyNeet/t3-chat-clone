use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::state::AppState;

pub mod enroll;
pub mod list;
pub mod remove;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/keys/enroll", post(enroll::handler))
        .route("/keys", get(list::handler))
        .route("/keys/{key_id}", delete(remove::handler))
}
