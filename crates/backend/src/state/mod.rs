use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{config::ModelsConfig, data::mongodb::MongoDataAdapter, search::WebSearch};
use ai::openai::{completions::OpenAICompletionDelta, streaming::OpenAIClient};
use model::{chat::Chat, key::ApiKey, message::ChatMessage, user::User};
use mongodb::{Client, IndexModel, bson::doc, options::IndexOptions};
use redis_om::redis::aio::MultiplexedConnection;
use serde::Serialize;
use uuid::Uuid;

pub struct AppState {
    openrouter: OpenAIClient,
    streams: Arc<Mutex<HashMap<Uuid, flume::Receiver<ApiDelta>>>>,
    database: Database,
    redis: MultiplexedConnection,
    hmac_key: Box<[u8]>,
    aes_key: Box<[u8]>,
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
        client.database("chat").create_collection("chats").await?;
        client
            .database("chat")
            .create_collection("messages")
            .await?;
        client
            .database("chat")
            .collection::<User>("messages")
            .create_index(IndexModel::builder().keys(doc! { "chat_id": 1 }).build())
            .await?;

        client
            .database("chat")
            .collection::<ApiKey>("keys")
            .create_index(IndexModel::builder().keys(doc! { "user_id": 1 }).build())
            .await?;

        let conn = redis.get_multiplexed_tokio_connection().await?;

        Ok(Self {
            openrouter,
            streams: Default::default(),
            database: Database {
                users: MongoDataAdapter::new(
                    client.clone(),
                    "chat".to_string(),
                    "users".to_string(),
                ),
                chats: MongoDataAdapter::new(
                    client.clone(),
                    "chat".to_string(),
                    "chats".to_string(),
                ),
                messages: MongoDataAdapter::new(
                    client.clone(),
                    "chat".to_string(),
                    "messages".to_string(),
                ),
                keys: MongoDataAdapter::new(client, "chat".to_string(), "keys".to_string()),
            },
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
}

pub enum ApiDelta {
    Chunk(OpenAICompletionDelta),
    Control(ControlChunk),
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum ControlChunk {
    Done { message: ChatMessage },
    WebSearchPerformed,
    ChatNameUpdated { name: String },
}

pub struct Database {
    pub users: MongoDataAdapter<User>,
    pub chats: MongoDataAdapter<Chat>,
    pub messages: MongoDataAdapter<ChatMessage>,
    pub keys: MongoDataAdapter<ApiKey>,
}
