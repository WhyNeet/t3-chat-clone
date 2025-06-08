use std::{env, process, sync::Arc};

use ai::openai::streaming::OpenAIClient;
use axum::Router;

use backend::{logger::Logger, routes, state::AppState};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let _log = Logger::init().unwrap_or_else(|e| {
        tracing::error!("An error occured during initialization: {e}");
        process::exit(1)
    });

    tracing::info!("Starting T3 Chat Clone backend.");

    let openrouter =
        OpenAIClient::new(env::var("OPENROUTER_KEY").expect("Missing OpenRouter API key"));
    let app_state = AppState::new(openrouter);

    let app = Router::new()
        .merge(routes::router())
        .with_state(Arc::new(app_state))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
