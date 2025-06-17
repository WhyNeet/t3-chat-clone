use std::env;

use anyhow::Context;
use redis_om::redis::aio::MultiplexedConnection;

pub struct CacheState {
    connection: MultiplexedConnection,
}

impl CacheState {
    pub async fn new() -> anyhow::Result<Self> {
        let uri = env::var("REDIS_URI").context("Missing Redis URI")?;
        let client = redis_om::Client::open(uri)?;
        let connection = client.get_multiplexed_tokio_connection().await?;

        Ok(Self { connection })
    }

    pub fn connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}
