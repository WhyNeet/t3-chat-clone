use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub mod login;
pub mod register;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register::handler))
        .route("/auth/login", post(login::handler))
}
