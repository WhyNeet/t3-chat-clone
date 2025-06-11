use std::io;

use anyhow::anyhow;
use futures::{AsyncBufReadExt, Stream, StreamExt, TryStreamExt};
use reqwest::Client;

use crate::openai::completions::{
    OpenAIChatCompletionRequest, OpenAIChatCompletionRequestReasoning, OpenAICompletionChunk,
    OpenAIMessage, ReasoningEffort,
};

#[derive(Debug, Clone)]
pub struct OpenAIClient {
    key: String,
}

impl OpenAIClient {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub async fn completion(
        self,
        model: String,
        messages: Vec<OpenAIMessage>,
        temperature: Option<f32>,
        reasoning_effort: Option<ReasoningEffort>,
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
        };

        let request = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .bearer_auth(self.key)
            .json(&openai_req_body)
            .send()
            .await?;
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
}
