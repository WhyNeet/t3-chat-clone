use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Sse, sse::Event},
};
use futures::StreamExt;
use model::message::ChatMessageContent;
use serde_json::json;
use uuid::Uuid;

use crate::{
    payload::chat::{ChatMessageContentPayload, ChatMessagePayload},
    state::AppState,
    streaming::{ApiDelta, ControlChunk},
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
                      content: message.content
                    .into_iter()
                    .map(|message| match message {
                        ChatMessageContent::Text { value } => {
                            ChatMessageContentPayload::Text { value }
                        }
                        ChatMessageContent::Image { id } => ChatMessageContentPayload::Image { id },
                        ChatMessageContent::Pdf { id } => ChatMessageContentPayload::Pdf { id },
                    })
                    .collect(),
                      model: message.model,
                      reasoning: message.reasoning,
                      role: message.role,
                      updated_memory: message.updated_memory,
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
