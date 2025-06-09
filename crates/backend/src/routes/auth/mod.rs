use std::sync::Arc;

use axum::{Router, routing::post};
use hmac::Hmac;
use sha2::Sha256;

use crate::state::AppState;

pub const SESSION_ID_COOKIE_NAME: &str = "sid";
pub const SESSION_EXPIRATION: usize = 2592000;
pub type HmacSha256 = Hmac<Sha256>;

pub mod login;
pub mod register;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register::handler))
        .route("/auth/login", post(login::handler))
}
