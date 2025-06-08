pub mod sse;

use std::sync::Arc;

use ai::openai::completions::OpenAIMessage;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use futures::TryStreamExt;
use serde_json::json;
use uuid::Uuid;

use crate::{
    payload::completion::PromptCompletionPayload,
    state::{ApiDelta, AppState},
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PromptCompletionPayload>,
) -> impl IntoResponse {
    let client = state.openrouter().clone();
    let Ok(stream) = client
        .completion(
            payload.model,
            vec![OpenAIMessage {
                content: payload.message,
                role: "user".to_string(),
            }],
            Some(0.7),
        )
        .await
    else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };
    tracing::info!("Created stream.");
    let mut stream = Box::pin(stream);

    let stream_id = Uuid::new_v4();

    let (tx, rx) = flume::unbounded();
    let task_state = Arc::clone(&state);
    tokio::spawn(async move {
        while let Ok(chunk) = stream.try_next().await {
            let Some(chunk) = chunk else { break };
            tx.send_async(ApiDelta::Chunk(chunk)).await.unwrap();
        }
        tx.send(ApiDelta::Done).unwrap();
        task_state.remove_stream(&stream_id);
    });

    state.insert_stream(stream_id, rx);

    (StatusCode::OK, Json(json!({ "stream_id": stream_id }))).into_response()
}
