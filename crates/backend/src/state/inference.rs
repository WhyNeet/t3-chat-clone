use std::env;

use ai::openai::client::OpenAIClient;
use anyhow::{Context, anyhow};
use serde::Serialize;

pub struct InferenceState {
    pub openrouter: OpenAIClient,
    pub chutes: OpenAIClient,
}

impl InferenceState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            openrouter: OpenAIClient::new(
                env::var("OPENROUTER_KEY").context("Missing OpenRouter API key")?,
                "https://openrouter.ai/api".to_string(),
            ),
            chutes: OpenAIClient::new(
                env::var("CHUTES_KEY").context("Missing Chutes API key")?,
                "https://llm.chutes.ai".to_string(),
            ),
        })
    }
}

#[derive(Debug, Serialize, Clone, Copy)]
pub enum InferenceProvider {
    OpenRouter,
    Chutes,
}

impl TryFrom<&str> for InferenceProvider {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "OpenRouter" => Ok(Self::OpenRouter),
            "Chutes" => Ok(Self::Chutes),
            other => Err(anyhow!("Invalid inference provider: {other}")),
        }
    }
}
