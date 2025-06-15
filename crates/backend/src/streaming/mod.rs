use ai::openai::completions::OpenAICompletionDelta;
use model::message::ChatMessage;
use serde::Serialize;

use crate::payload::memories::MemoryPayload;

pub enum ApiDelta {
    Chunk(OpenAICompletionDelta),
    Control(ControlChunk),
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum ControlChunk {
    Done { message: ChatMessage },
    WebSearchPerformed,
    ChatNameUpdated { name: String },
    MemoryAdded { memory: MemoryPayload },
    InferenceError { code: u16 },
}
