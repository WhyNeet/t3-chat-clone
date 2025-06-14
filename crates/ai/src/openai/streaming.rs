use std::io;

use anyhow::anyhow;
use futures::{AsyncBufReadExt, Stream, StreamExt, TryStreamExt};
use reqwest::{Client, StatusCode};

use crate::openai::completions::{
    OpenAIChatCompletionRequest, OpenAIChatCompletionRequestReasoning, OpenAICompletionChunk,
    OpenAIMessage, OpenAIPromptCompletionRequest, OpenAIPromptCompletionResponse, ReasoningEffort,
};

use super::completions::OpenRouterRequestPlugins;

#[derive(Debug, Clone)]
pub struct OpenAIClient {
    key: String,
    endpoint: String,
}

impl OpenAIClient {
    pub fn new(key: String, endpoint: String) -> Self {
        Self { key, endpoint }
    }

    pub async fn completion(
        self,
        model: String,
        messages: Vec<OpenAIMessage>,
        temperature: Option<f32>,
        reasoning_effort: Option<ReasoningEffort>,
        plugins: Option<OpenRouterRequestPlugins>,
    ) -> anyhow::Result<impl Stream<Item = anyhow::Result<OpenAICompletionChunk>>> {
        let client = Client::new();

        let openai_req_body = OpenAIChatCompletionRequest {
            model,
            messages,
            stream: true,
            temperature,
            max_tokens: None,
            reasoning: reasoning_effort
                .map(|effort| OpenAIChatCompletionRequestReasoning { effort }),
            // plugins,
        };

        let request = client
            .post(self.endpoint)
            .bearer_auth(self.key)
            .json(&openai_req_body)
            .send()
            .await?;
        if request.status() != StatusCode::OK {
            anyhow::bail!(request.status())
        }
        let bytes_stream = request.bytes_stream();

        Ok(bytes_stream
            .map_err(|e| io::Error::other(e))
            .into_async_read()
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        // OpenAI sends lines like "data: {json}" or "data: [DONE]"
                        if line.starts_with("data: ") {
                            let json_str = &line[6..]; // Remove "data: " prefix
                            if json_str == "[DONE]" {
                                return None; // Signal to terminate the stream
                            }

                            match serde_json::from_str::<OpenAICompletionChunk>(json_str) {
                                Ok(chunk) => Some(Ok(chunk)),
                                Err(e) => Some(Err(anyhow!("error: {e}"))),
                            }
                        } else {
                            // skip empty line
                            None
                        }
                    }
                    Err(e) => Some(Err(anyhow!("error: {e}"))),
                }
            }))
    }

    pub async fn prompt_completion_non_streaming(
        self,
        model: String,
        prompt: String,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> anyhow::Result<String> {
        let client = Client::new();

        let openai_req_body = OpenAIPromptCompletionRequest {
            model,
            prompt,
            stream: false,
            temperature,
            max_tokens,
            reasoning: None,
        };

        let response = client
            .post(self.endpoint)
            .bearer_auth(self.key)
            .json(&openai_req_body)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            anyhow::bail!(response.status())
        }

        let mut response: OpenAIPromptCompletionResponse = response.json().await?;

        // dbg!(&response);

        Ok(response.choices.remove(0).text.trim().to_string())
    }
}
