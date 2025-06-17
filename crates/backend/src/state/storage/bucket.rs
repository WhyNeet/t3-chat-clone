use std::time::Duration;

use mongodb::{
    Client,
    gridfs::GridFsBucket,
    options::{GridFsBucketOptions, WriteConcern},
};

pub struct BucketState {
    bucket: GridFsBucket,
}

impl BucketState {
    pub async fn new(client: Client) -> anyhow::Result<Self> {
        let gridfs_opts = GridFsBucketOptions::builder()
            .bucket_name("attachments".to_string())
            .write_concern(
                WriteConcern::builder()
                    .w_timeout(Duration::new(5, 0))
                    .build(),
            )
            .build();

        let bucket = client.database("chat").gridfs_bucket(gridfs_opts);

        Ok(Self { bucket })
    }

    pub fn gridfs(&self) -> &GridFsBucket {
        &self.bucket
    }
}
