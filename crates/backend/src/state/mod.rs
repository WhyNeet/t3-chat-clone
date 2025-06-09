use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::data::mongodb::MongoDataAdapter;
use ai::openai::{completions::OpenAICompletionChunk, streaming::OpenAIClient};
use model::user::User;
use mongodb::{Client, IndexModel, bson::doc, options::IndexOptions};
use redis_om::redis::aio::MultiplexedConnection;
use uuid::Uuid;

pub struct AppState {
    openrouter: OpenAIClient,
    streams: Arc<Mutex<HashMap<Uuid, flume::Receiver<ApiDelta>>>>,
    users: MongoDataAdapter<User>,
    redis: MultiplexedConnection,
    hmac_key: Box<[u8]>,
}

impl AppState {
    pub async fn new(
        openrouter: OpenAIClient,
        client: Client,
        redis: redis_om::Client,
        hmac_key: Box<[u8]>,
    ) -> anyhow::Result<Self> {
        client.database("chat").create_collection("users").await?;
        client
            .database("chat")
            .collection::<User>("users")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "email": -1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await?;

        let conn = redis.get_multiplexed_tokio_connection().await?;

        Ok(Self {
            openrouter,
            streams: Default::default(),
            users: MongoDataAdapter::new(client, "chat".to_string(), "users".to_string()),
            redis: conn,
            hmac_key,
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

    pub fn remove_stream(&self, id: &Uuid) {
        self.streams.lock().unwrap().remove(id);
    }

    pub fn users(&self) -> &MongoDataAdapter<User> {
        &self.users
    }

    pub fn redis(&self) -> MultiplexedConnection {
        self.redis.clone()
    }

    pub fn hmac_key(&self) -> &[u8] {
        &self.hmac_key
    }
}

pub enum ApiDelta {
    Chunk(OpenAICompletionChunk),
    Done,
}
