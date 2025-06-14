use chrono::Utc;
use model::message::Role;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatPayload {
    #[serde(serialize_with = "super::serialize_oid")]
    pub id: ObjectId,
    pub name: Option<String>,
    #[serde(serialize_with = "super::serialize_oid")]
    pub user_id: ObjectId,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessagePayload {
    #[serde(serialize_with = "super::serialize_oid")]
    pub id: ObjectId,
    pub content: Vec<ChatMessageContentPayload>,
    pub model: Option<String>,
    pub reasoning: Option<String>,
    pub role: Role,
    #[serde(serialize_with = "super::serialize_oid")]
    pub chat_id: ObjectId,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(tag = "type")]
pub enum ChatMessageContentPayload {
    Text {
        value: String,
    },
    Image {
        #[serde(serialize_with = "super::serialize_oid")]
        id: ObjectId,
    },
    Pdf {
        #[serde(serialize_with = "super::serialize_oid")]
        id: ObjectId,
    },
}
