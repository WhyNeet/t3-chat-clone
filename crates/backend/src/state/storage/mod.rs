pub mod bucket;
pub mod cache;
pub mod database;

use std::env;

use anyhow::Context;
use database::DatabaseState;
use mongodb::{Client, options::ClientOptions};

use crate::state::storage::{bucket::BucketState, cache::CacheState};

pub struct StorageState {
    database: DatabaseState,
    cache: CacheState,
    bucket: BucketState,
}

impl StorageState {
    pub async fn new() -> anyhow::Result<Self> {
        let client_uri = env::var("MONGODB_URI").context("Missing MongoDB URI")?;
        let options = ClientOptions::parse(&client_uri).await.unwrap();
        let client = Client::with_options(options).unwrap();

        Ok(Self {
            database: DatabaseState::new(client.clone()).await?,
            cache: CacheState::new().await?,
            bucket: BucketState::new(client).await?,
        })
    }

    pub fn database(&self) -> &DatabaseState {
        &self.database
    }

    pub fn cache(&self) -> &CacheState {
        &self.cache
    }

    pub fn bucket(&self) -> &BucketState {
        &self.bucket
    }
}
