use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChatCreatePayload {

// }

pub async fn handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {}
