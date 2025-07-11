use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserUpload {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub chat_id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub content_type: String,
    pub is_sent: bool,
}
