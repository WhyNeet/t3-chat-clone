pub mod state;

use std::{env, sync::Arc};

use ai::openai::streaming::OpenAIClient;
use axum::Router;

use crate::state::AppState;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let openrouter =
        OpenAIClient::new(env::var("OPENROUTER_KEY").expect("Missing OpenRouter API key."));
    let app_state = AppState::new(openrouter);

    let app = Router::new().with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
