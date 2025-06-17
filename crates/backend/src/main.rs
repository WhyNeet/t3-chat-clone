use std::{process, sync::Arc};

use axum::Router;

use backend::{logger::Logger, middleware::auth::AuthMiddlewareLayer, routes, state::AppState};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let _log = Logger::init().unwrap_or_else(|e| {
        tracing::error!("An error occured during initialization: {e}");
        process::exit(1)
    });

    tracing::info!("Starting T3 Chat Clone backend.");

    let app_state = AppState::new().await.unwrap();
    let app_state = Arc::new(app_state);

    let app = Router::new()
        .merge(routes::router())
        .with_state(Arc::clone(&app_state))
        .layer(CorsLayer::very_permissive())
        .layer(AuthMiddlewareLayer { state: app_state });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
