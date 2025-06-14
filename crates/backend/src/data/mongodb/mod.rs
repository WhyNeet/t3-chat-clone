use std::marker::PhantomData;

use mongodb::{
    Client,
    bson::{Document, doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};

pub struct MongoDataAdapter<Entity: Serialize + for<'a> Deserialize<'a> + Send + Sync> {
    client: Client,
    db: String,
    collection: String,
    d: PhantomData<Entity>,
}

impl<Entity: Serialize + for<'a> Deserialize<'a> + Send + Sync> MongoDataAdapter<Entity> {
    pub fn new(client: Client, db: String, collection: String) -> Self {
        Self {
            client,
            db,
            collection,
            d: PhantomData,
        }
    }
}

impl<Entity: Serialize + for<'a> Deserialize<'a> + Send + Sync> MongoDataAdapter<Entity> {
    pub async fn get_by_id(&self, id: ObjectId) -> anyhow::Result<Option<Entity>> {
        Ok(self
            .client
            .database(&self.db)
            .collection::<Entity>(&self.collection)
            .find_one(doc! { "_id": id })
            .await?)
    }
    pub async fn get_many(&self, doc: Document) -> anyhow::Result<mongodb::Cursor<Entity>> {
        Ok(self
            .client
            .database(&self.db)
            .collection::<Entity>(&self.collection)
            .find(doc)
            .await?)
    }
    pub async fn get_many_sorted(
        &self,
        doc: Document,
        sort: Document,
    ) -> anyhow::Result<mongodb::Cursor<Entity>> {
        Ok(self
            .client
            .database(&self.db)
            .collection::<Entity>(&self.collection)
            .find(doc)
            .sort(sort)
            .await?)
    }
    pub async fn get(&self, doc: Document) -> anyhow::Result<Option<Entity>> {
        Ok(self
            .client
            .database(&self.db)
            .collection::<Entity>(&self.collection)
            .find_one(doc)
            .await?)
    }
    pub async fn create(&self, entity: Entity) -> anyhow::Result<ObjectId> {
        let entity = self
            .client
            .database(&self.db)
            .collection(&self.collection)
            .insert_one(entity)
            .await?;

        Ok(entity.inserted_id.as_object_id().unwrap())
    }
    pub async fn update(&self, id: ObjectId, update: Document) -> anyhow::Result<()> {
        self.client
            .database(&self.db)
            .collection::<Entity>(&self.collection)
            .update_one(doc! { "_id": id }, update)
            .await?;

        Ok(())
    }
    pub async fn delete(&self, id: ObjectId) -> anyhow::Result<()> {
        self.client
            .database(&self.db)
            .collection::<Entity>(&self.collection)
            .delete_one(doc! { "_id": id })
            .await?;

        Ok(())
    }
}
