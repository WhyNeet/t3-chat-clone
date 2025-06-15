use ai::openai::completions::OpenAICompletionDelta;
use model::message::ChatMessage;
use serde::Serialize;

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
    MemoryAdded { memory: String },
    InferenceError { code: u16 },
}
