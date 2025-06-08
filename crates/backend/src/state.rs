use ai::openai::streaming::OpenAIClient;

pub struct AppState {
    openrouter: OpenAIClient,
}

impl AppState {
    pub fn new(openrouter: OpenAIClient) -> Self {
        Self { openrouter }
    }

    pub fn openrouter(&self) -> &OpenAIClient {
        &self.openrouter
    }
}
