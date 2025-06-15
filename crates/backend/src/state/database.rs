use model::{
    chat::Chat, key::ApiKey, memory::Memory, message::ChatMessage, upload::UserUpload, user::User,
};
use mongodb::{Client, IndexModel, bson::doc, options::IndexOptions};

use crate::data::mongodb::MongoDataAdapter;

pub struct Database {
    pub users: MongoDataAdapter<User>,
    pub chats: MongoDataAdapter<Chat>,
    pub messages: MongoDataAdapter<ChatMessage>,
    pub keys: MongoDataAdapter<ApiKey>,
    pub uploads: MongoDataAdapter<UserUpload>,
    pub memories: MongoDataAdapter<Memory>,
}

impl Database {
    pub async fn new(client: Client) -> anyhow::Result<Self> {
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
            .collection::<ChatMessage>("chats")
            .create_index(IndexModel::builder().keys(doc! { "user_id": 1 }).build())
            .await?;
        client
            .database("chat")
            .create_collection("messages")
            .await?;
        client
            .database("chat")
            .collection::<ChatMessage>("messages")
            .create_index(IndexModel::builder().keys(doc! { "chat_id": 1 }).build())
            .await?;

        client
            .database("chat")
            .collection::<ApiKey>("keys")
            .create_index(IndexModel::builder().keys(doc! { "user_id": 1 }).build())
            .await?;
        client
            .database("chat")
            .collection::<UserUpload>("uploads")
            .create_index(IndexModel::builder().keys(doc! { "chat_id": 1 }).build())
            .await?;
        client
            .database("chat")
            .collection::<Memory>("memories")
            .create_index(IndexModel::builder().keys(doc! { "user_id": 1 }).build())
            .await?;

        Ok(Self {
            users: MongoDataAdapter::new(client.clone(), "chat".to_string(), "users".to_string()),
            chats: MongoDataAdapter::new(client.clone(), "chat".to_string(), "chats".to_string()),
            messages: MongoDataAdapter::new(
                client.clone(),
                "chat".to_string(),
                "messages".to_string(),
            ),
            keys: MongoDataAdapter::new(client.clone(), "chat".to_string(), "keys".to_string()),
            uploads: MongoDataAdapter::new(
                client.clone(),
                "chat".to_string(),
                "uploads".to_string(),
            ),
            memories: MongoDataAdapter::new(client, "chat".to_string(), "memories".to_string()),
        })
    }
}
