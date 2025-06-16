use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    config::ModelsConfig, search::WebSearch, state::database::Database, streaming::ApiDelta,
};
use ai::openai::client::OpenAIClient;
use mongodb::{
    Client,
    gridfs::GridFsBucket,
    options::{GridFsBucketOptions, WriteConcern},
};
use redis_om::redis::aio::MultiplexedConnection;
use uuid::Uuid;

pub mod database;

pub struct AppState {
    openrouter: OpenAIClient,
    streams: Arc<Mutex<HashMap<Uuid, flume::Receiver<ApiDelta>>>>,
    database: Database,
    redis: MultiplexedConnection,
    hmac_key: Box<[u8]>,
    aes_key: Box<[u8]>,
    bucket: GridFsBucket,
    models: ModelsConfig,
    search: WebSearch,
}

impl AppState {
    pub async fn new(
        openrouter: OpenAIClient,
        client: Client,
        redis: redis_om::Client,
        aes_key: Box<[u8]>,
        hmac_key: Box<[u8]>,
        search: WebSearch,
    ) -> anyhow::Result<Self> {
        let gridfs_opts = GridFsBucketOptions::builder()
            .bucket_name("attachments".to_string())
            .write_concern(
                WriteConcern::builder()
                    .w_timeout(Duration::new(5, 0))
                    .build(),
            )
            .build();
        let bucket = client.database("chat").gridfs_bucket(gridfs_opts);

        let conn = redis.get_multiplexed_tokio_connection().await?;

        Ok(Self {
            openrouter,
            streams: Default::default(),
            database: Database::new(client).await?,
            bucket,
            redis: conn,
            aes_key,
            hmac_key,
            models: ModelsConfig::new(),
            search,
        })
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

    pub fn remove_stream(&self, id: &Uuid) -> bool {
        self.streams.lock().unwrap().remove(id).is_some()
    }

    pub fn redis(&self) -> MultiplexedConnection {
        self.redis.clone()
    }

    pub fn hmac_key(&self) -> &[u8] {
        &self.hmac_key
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn models(&self) -> &ModelsConfig {
        &self.models
    }

    pub fn search(&self) -> &WebSearch {
        &self.search
    }

    pub fn aes_key(&self) -> &[u8] {
        &self.aes_key
    }

    pub fn bucket(&self) -> &GridFsBucket {
        &self.bucket
    }
}
