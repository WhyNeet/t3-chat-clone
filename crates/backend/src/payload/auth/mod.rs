use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct UserPayload {
    #[serde(serialize_with = "super::serialize_oid")]
    pub id: ObjectId,
    pub email: String,
}
