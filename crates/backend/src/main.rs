use std::{env, process, sync::Arc};

use ai::openai::streaming::OpenAIClient;
use axum::Router;

use backend::{logger::Logger, routes, state::AppState};
use mongodb::{Client, options::ClientOptions};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let _log = Logger::init().unwrap_or_else(|e| {
        tracing::error!("An error occured during initialization: {e}");
        process::exit(1)
    });

    tracing::info!("Starting T3 Chat Clone backend.");

    let mongodb = {
        let client_uri = env::var("MONGODB_URI").expect("Missing MongoDB URI");

        let options = ClientOptions::parse(&client_uri).await.unwrap();
        Client::with_options(options).unwrap()
    };
    let redis = {
        let uri = env::var("REDIS_URI").expect("Missing Redis URI");
        redis_om::Client::open(uri).unwrap()
    };
    let openrouter =
        OpenAIClient::new(env::var("OPENROUTER_KEY").expect("Missing OpenRouter API key"));
    let session_key = env::var("SESSION_SECRET_KEY")
        .expect("Missing session secret key")
        .as_bytes()
        .to_vec()
        .into_boxed_slice();
    let app_state = AppState::new(openrouter, mongodb, redis, session_key)
        .await
        .unwrap();

    let app = Router::new()
        .merge(routes::router())
        .with_state(Arc::new(app_state))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
