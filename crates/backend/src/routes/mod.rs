pub mod auth;
pub mod chats;
pub mod completion;
pub mod files;
pub mod keys;
pub mod service;
pub mod users;

use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(service::router())
        .merge(completion::router())
        .merge(auth::router())
        .merge(users::router())
        .merge(chats::router())
        .merge(keys::router())
        .merge(files::router())
}
