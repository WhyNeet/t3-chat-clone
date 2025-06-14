use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{config::ModelsConfig, data::mongodb::MongoDataAdapter, search::WebSearch};
use ai::openai::{completions::OpenAICompletionDelta, streaming::OpenAIClient};
use model::{chat::Chat, key::ApiKey, message::ChatMessage, upload::UserUpload, user::User};
use mongodb::{
    Client, IndexModel,
    bson::doc,
    gridfs::GridFsBucket,
    options::{GridFsBucketOptions, IndexOptions, WriteConcern},
};
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
        client
            .database("chat")
            .collection::<ApiKey>("uploads")
            .create_index(IndexModel::builder().keys(doc! { "chat_id": 1 }).build())
            .await?;

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
                keys: MongoDataAdapter::new(client.clone(), "chat".to_string(), "keys".to_string()),
                uploads: MongoDataAdapter::new(client, "chat".to_string(), "uploads".to_string()),
            },
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
    InferenceError { code: u16 },
}

pub struct Database {
    pub users: MongoDataAdapter<User>,
    pub chats: MongoDataAdapter<Chat>,
    pub messages: MongoDataAdapter<ChatMessage>,
    pub keys: MongoDataAdapter<ApiKey>,
    pub uploads: MongoDataAdapter<UserUpload>,
}
