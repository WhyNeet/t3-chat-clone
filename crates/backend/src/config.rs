use serde::Serialize;

pub struct ModelsConfig {
    free_models: Vec<Model>,
}

impl ModelsConfig {
    pub fn new() -> Self {
        Self {
            free_models: vec![
                Model {
                    identifier: "google/gemini-2.0-flash-exp:free".to_string(),
                    name: "Gemini 2.0 Flash Experimental".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: false,
                },
                Model {
                    identifier: "meta-llama/llama-4-maverick:free".to_string(),
                    name: "Llama 4 Maverick".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: false,
                },
                Model {
                    identifier: "deepseek/deepseek-r1-distill-llama-70b:free".to_string(),
                    name: "DeepSeek R1 Distill Llama 70B".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: true,
                },
                Model {
                    identifier: "meta-llama/llama-4-scout:free".to_string(),
                    name: "Llama 4 Scout".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: false,
                },
                Model {
                    identifier: "nvidia/llama-3.1-nemotron-ultra-253b-v1:free".to_string(),
                    name: "Llama 3.1 Nemotron Ultra".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: true,
                },
                Model {
                    identifier: "google/gemma-3-27b-it:free".to_string(),
                    name: "Gemma 3".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: false,
                },
                Model {
                    identifier: "deepseek/deepseek-chat-v3-0324:free".to_string(),
                    name: "DeepSeek V3".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: false,
                },
                Model {
                    identifier: "deepseek/deepseek-r1-0528:free".to_string(),
                    name: "DeepSeek R1".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: true,
                },
                Model {
                    identifier: "tngtech/deepseek-r1t-chimera:free".to_string(),
                    name: "DeepSeek R1T Chimera".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: true,
                },
                Model {
                    identifier: "qwen/qwen3-235b-a22b:free".to_string(),
                    name: "Qwen 235B A22B".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: true,
                },
                Model {
                    identifier: "qwen/qwq-32b:free".to_string(),
                    name: "QWQ 32B".to_string(),
                    api_kind: ModelApiKind::OpenAI,
                    base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                    is_reasoning: true,
                },
            ],
        }
    }

    pub fn free_models(&self) -> &[Model] {
        &self.free_models
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub name: String,
    pub identifier: String,
    pub api_kind: ModelApiKind,
    pub base_url: String,
    pub is_reasoning: bool,
}

#[derive(Debug, Clone, Serialize)]
pub enum ModelApiKind {
    #[serde(rename = "openai")]
    OpenAI,
}
