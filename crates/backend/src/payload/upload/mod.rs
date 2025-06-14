use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct UserUploadPayload {
    #[serde(serialize_with = "super::serialize_oid")]
    pub id: ObjectId,
    #[serde(serialize_with = "super::serialize_oid")]
    pub user_id: ObjectId,
    #[serde(serialize_with = "super::serialize_oid")]
    pub chat_id: ObjectId,
    pub content_type: String,
}
