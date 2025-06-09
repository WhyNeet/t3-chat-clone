use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct UserPayload {
    #[serde(serialize_with = "serialize_oid")]
    pub id: ObjectId,
    pub email: String,
}

fn serialize_oid<S>(oid: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&oid.to_hex())
}
