pub mod completion;
pub mod service;

use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(service::router())
        .merge(completion::router())
}
