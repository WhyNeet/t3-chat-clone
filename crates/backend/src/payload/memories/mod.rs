use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MemoryPayload {
    #[serde(serialize_with = "super::serialize_oid")]
    pub id: ObjectId,
    pub content: String,
}
