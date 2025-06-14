use mongodb::bson::oid::ObjectId;
use serde::Serializer;

pub mod auth;
pub mod chat;
pub mod upload;

pub fn serialize_oid<S>(oid: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&oid.to_hex())
}

pub fn serialize_option_oid<S>(oid: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(oid) = oid {
        serializer.serialize_str(&oid.to_hex())
    } else {
        serializer.serialize_none()
    }
}
