use std::{sync::Arc, time::Duration};

use ai::openai::completions::OpenAIMessage;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Sse, sse::Event},
};
use futures::StreamExt;

use crate::{payload::completion::PromptCompletionPayload, state::AppState};

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
    let stream = stream.filter_map(|chunk| async {
        println!("chunk: {chunk:?}");
        Some(
            Event::default().json_data(
                chunk
                    .map(|mut c| c.choices.remove(0).delta)
                    .map_err(|e| e.to_string()),
            ),
        )
    });

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(1))
                .text("keep-alive-text"),
        )
        .into_response()
}
