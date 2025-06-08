use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ai::openai::{completions::OpenAICompletionChunk, streaming::OpenAIClient};
use uuid::Uuid;

pub struct AppState {
    openrouter: OpenAIClient,
    streams: Arc<Mutex<HashMap<Uuid, flume::Receiver<ApiDelta>>>>,
}

impl AppState {
    pub fn new(openrouter: OpenAIClient) -> Self {
        Self {
            openrouter,
            streams: Default::default(),
        }
    }

    pub fn openrouter(&self) -> &OpenAIClient {
        &self.openrouter
    }

    pub fn insert_stream(&self, id: Uuid, recv: flume::Receiver<ApiDelta>) {
        self.streams.lock().unwrap().insert(id, recv);
    }

    pub fn get_stream(&self, id: &Uuid) -> Option<flume::Receiver<ApiDelta>> {
        self.streams.lock().unwrap().get(id).cloned()
    }

    pub fn remove_stream(&self, id: &Uuid) {
        self.streams.lock().unwrap().remove(id);
    }
}

pub enum ApiDelta {
    Chunk(OpenAICompletionChunk),
    Done,
}
