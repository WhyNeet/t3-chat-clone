use bson::oid::ObjectId;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub user_id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: chrono::DateTime<Utc>,
}
