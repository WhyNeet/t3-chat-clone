use bson::oid::ObjectId;
use redis_om::HashModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiKey {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub provider: String,
    pub key: String,
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, HashModel)]
pub struct UserApiKey {
    /// User ID
    pub id: String,
    pub key_id: String,
    pub key: String,
}
