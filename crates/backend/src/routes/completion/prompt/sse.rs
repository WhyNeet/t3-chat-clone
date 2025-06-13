use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Sse, sse::Event},
};
use futures::StreamExt;
use serde_json::json;
use uuid::Uuid;

use crate::{
    payload::chat::ChatMessagePayload,
    state::{ApiDelta, AppState, ControlChunk},
};

pub async fn handler(
    Path(stream_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let Some(recv) = state.get_stream(&stream_id) else {
        return (StatusCode::BAD_REQUEST).into_response();
    };

    let stream = recv.into_stream().map(move |delta| match delta {
        ApiDelta::Chunk(chunk) => Event::default().json_data(chunk),
        ApiDelta::Control(control) => match control {
            ControlChunk::Done { message } => {
                state.remove_stream(&stream_id);
                tracing::debug!("Streaming finished.");
                Event::default().json_data(
                    json!({ "control": { "kind": "Done", "message": ChatMessagePayload {
                      id: message.id.unwrap(),
                      chat_id: message.chat_id,
                      content: message.content,
                      model: message.model,
                      reasoning: message.reasoning,
                      role: message.role,
                      timestamp: message.timestamp
                } } }),
                )
            }
            other => Event::default().json_data(json!({ "control": other })),
        },
    });

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)))
        .into_response()
}
