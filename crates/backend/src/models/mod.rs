use serde::Serialize;

use crate::state::inference::InferenceProvider;

pub struct ModelsConfig {
    free_models: Vec<Model>,
    paid_models: Vec<Model>,
}

impl ModelsConfig {
    pub fn new() -> Self {
        Self {
            free_models: vec![
                Model {
                    identifier: "google/gemini-2.0-flash-exp:free".to_string(),
                    name: "Gemini 2.0 Flash Experimental".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: false,
                    author: "Google".to_string(),
                },
                Model {
                    identifier: "meta-llama/llama-4-maverick:free".to_string(),
                    name: "Llama 4 Maverick".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: false,
                    author: "Meta".to_string(),
                },
                Model {
                    identifier: "deepseek/deepseek-r1-distill-llama-70b:free".to_string(),
                    name: "DeepSeek R1 Distill Llama 70B".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "DeepSeek".to_string(),
                },
                Model {
                    identifier: "meta-llama/llama-4-scout:free".to_string(),
                    name: "Llama 4 Scout".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: false,
                    author: "Meta".to_string(),
                },
                Model {
                    identifier: "nvidia/llama-3.1-nemotron-ultra-253b-v1:free".to_string(),
                    name: "Llama 3.1 Nemotron Ultra".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "NVIDIA".to_string(),
                },
                Model {
                    identifier: "google/gemma-3-27b-it:free".to_string(),
                    name: "Gemma 3".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: false,
                    author: "Google".to_string(),
                },
                Model {
                    identifier: "deepseek/deepseek-chat-v3-0324:free".to_string(),
                    name: "DeepSeek V3".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: false,
                    author: "DeepSeek".to_string(),
                },
                Model {
                    identifier: "deepseek/deepseek-r1-0528:free".to_string(),
                    name: "DeepSeek R1".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "DeepSeek".to_string(),
                },
                Model {
                    identifier: "tngtech/deepseek-r1t-chimera:free".to_string(),
                    name: "DeepSeek R1T Chimera".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "TNG".to_string(),
                },
                Model {
                    identifier: "qwen/qwen3-235b-a22b:free".to_string(),
                    name: "Qwen 235B A22B".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Qwen".to_string(),
                },
                Model {
                    identifier: "qwen/qwq-32b:free".to_string(),
                    name: "QWQ 32B".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Qwen".to_string(),
                },
            ],
            paid_models: vec![
                Model {
                    identifier: "anthropic/claude-sonnet-4".to_string(),
                    name: "Claude Sonnet 4".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Anthropic".to_string(),
                },
                Model {
                    identifier: "anthropic/claude-opus-4".to_string(),
                    name: "Claude Opus 4".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Anthropic".to_string(),
                },
                Model {
                    identifier: "google/gemini-2.5-pro-preview".to_string(),
                    name: "Gemini 2.5 Pro Preview".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Google".to_string(),
                },
                Model {
                    identifier: "openai/gpt-4o-mini".to_string(),
                    name: "GPT-4o-mini".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "OpenAI".to_string(),
                },
                Model {
                    identifier: "google/gemini-2.5-flash-preview".to_string(),
                    name: "Gemini 2.5 Flash Preview".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Google".to_string(),
                },
                Model {
                    identifier: "google/gemini-2.5-flash-preview-05-20:thinking".to_string(),
                    name: "Gemini 2.5 Flash Preview (thinking)".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Google".to_string(),
                },
                Model {
                    identifier: "meta-llama/llama-3.1-70b-instruct".to_string(),
                    name: "Llama 3.1 70B Instruct".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Meta".to_string(),
                },
                Model {
                    identifier: "perplexity/llama-3.1-sonar-large-128k-online".to_string(),
                    name: "Llama 3.1 Sonar 70B Online".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "Perplexity".to_string(),
                },
                Model {
                    identifier: "openai/gpt-4-turbo".to_string(),
                    name: "GPT-4 Turbo".to_string(),
                    provider: InferenceProvider::OpenRouter,
                    is_reasoning: true,
                    author: "OpenAI".to_string(),
                },
            ],
        }
    }

    pub fn free_models(&self) -> &[Model] {
        &self.free_models
    }

    pub fn paid_models(&self) -> &[Model] {
        &self.paid_models
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub name: String,
    pub identifier: String,
    pub provider: InferenceProvider,
    pub is_reasoning: bool,
    pub author: String,
}
