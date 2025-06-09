use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::data::mongodb::MongoDataAdapter;
use ai::openai::{completions::OpenAICompletionChunk, streaming::OpenAIClient};
use model::user::User;
use mongodb::{Client, IndexModel, bson::doc, options::IndexOptions};
use uuid::Uuid;

pub struct AppState {
    openrouter: OpenAIClient,
    streams: Arc<Mutex<HashMap<Uuid, flume::Receiver<ApiDelta>>>>,
    users: MongoDataAdapter<User>,
}

impl AppState {
    pub async fn new(openrouter: OpenAIClient, client: Client) -> anyhow::Result<Self> {
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

        Ok(Self {
            openrouter,
            streams: Default::default(),
            users: MongoDataAdapter::new(client, "chat".to_string(), "users".to_string()),
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
}

pub enum ApiDelta {
    Chunk(OpenAICompletionChunk),
    Done,
}
