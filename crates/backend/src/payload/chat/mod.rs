use chrono::Utc;
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
