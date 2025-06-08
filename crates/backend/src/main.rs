use std::env;

use futures::StreamExt;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let openrouter_key = env::var("OPENROUTER_KEY").unwrap();

    let client = Client::new();

    let openai_req_body = OpenAIChatCompletionRequest {
        model: "google/gemini-2.0-flash-exp:free".to_string(),
        messages: vec![OpenAIMessage {
            role: "user".to_string(),
            content: "What is bitcoin?".to_string(),
        }],
        stream: true, // IMPORTANT: Enable streaming from OpenAI
        temperature: Some(0.7),
        max_tokens: None,
    };

    let request = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(openrouter_key)
        .json(&openai_req_body);

    let mut es = EventSource::new(request).unwrap();

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Message(message)) => {
                match serde_json::from_str::<OpenAICompletionChunk>(&message.data) {
                    Ok(chunk) => {
                        if let Some(ref content) = chunk.choices[0].delta.content {
                            print!("{}", content); // Stream tokens to stdout
                        }
                    }
                    Err(_) => {
                        // Skip invalid chunks (could be the final "[DONE]" event)
                    }
                }
            }
            Ok(Event::Open) => {} // Connection opened
            Err(reqwest_eventsource::Error::StreamEnded) => {
                println!("\nDONE");
                break;
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
                break;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub stream: bool, // Crucial for streaming responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

// --- OpenAI API Streaming Response Chunk ---
// This is what you get back from OpenAI with `stream: true`
// Each chunk is a JSON object.
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
    pub delta: OpenAICompletionDelta, // Contains the actual token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletionDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>, // The actual token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
