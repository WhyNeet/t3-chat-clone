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

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct ChatMessagePayload {
    #[serde(serialize_with = "super::serialize_oid")]
    pub id: ObjectId,
    pub content: String,
    pub reasoning: Option<String>,
    pub role: Role,
    #[serde(serialize_with = "super::serialize_oid")]
    pub chat_id: ObjectId,
    pub timestamp: chrono::DateTime<Utc>,
}
