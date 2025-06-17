use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use crate::{
    models::ModelsConfig,
    state::{crypto::CryptoState, inference::InferenceState, storage::StorageState},
    streaming::ApiDelta,
};
use anyhow::Context;
use search::{SearchClient, serper::SerperSearchClient};
use uuid::Uuid;

pub mod crypto;
pub mod inference;
pub mod storage;

pub struct AppState {
    inference: InferenceState,
    streams: Arc<Mutex<HashMap<Uuid, flume::Receiver<ApiDelta>>>>,
    storage: StorageState,
    crypto: CryptoState,
    models: ModelsConfig,
    search: Arc<dyn SearchClient>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            inference: InferenceState::new()?,
            streams: Default::default(),
            storage: StorageState::new().await?,
            crypto: CryptoState::new()?,
            models: ModelsConfig::new(),
            search: Arc::new(SerperSearchClient::new(
                env::var("SERPER_KEY").context("Missing OpenRouter API key")?,
            )),
        })
    }

    pub fn insert_stream(&self, id: Uuid, recv: flume::Receiver<ApiDelta>) {
        self.streams.lock().unwrap().insert(id, recv);
    }

    pub fn get_stream(&self, id: &Uuid) -> Option<flume::Receiver<ApiDelta>> {
        self.streams.lock().unwrap().get(id).cloned()
    }

    pub fn remove_stream(&self, id: &Uuid) -> bool {
        self.streams.lock().unwrap().remove(id).is_some()
    }

    pub fn models(&self) -> &ModelsConfig {
        &self.models
    }

    pub fn inference(&self) -> &InferenceState {
        &self.inference
    }

    pub fn search(&self) -> &dyn SearchClient {
        self.search.as_ref()
    }

    pub fn storage(&self) -> &StorageState {
        &self.storage
    }

    pub fn crypto(&self) -> &CryptoState {
        &self.crypto
    }
}
