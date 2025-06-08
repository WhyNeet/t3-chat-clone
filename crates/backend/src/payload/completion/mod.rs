use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompletionPayload {
    pub message: String,
    pub model: String,
}
