use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<OpenAIChatCompletionRequestReasoning>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequestReasoning {
    pub effort: ReasoningEffort,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<OpenAICompletionChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletionChoice {
    pub index: u32,
    pub delta: OpenAICompletionDelta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletionDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReasoningEffort {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIPromptCompletionRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<OpenAIChatCompletionRequestReasoning>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIPromptCompletionResponse {
    pub id: String,
    pub choices: Vec<OpenAIPromptCompletionChoice>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIPromptCompletionChoice {
    pub text: String,
}
