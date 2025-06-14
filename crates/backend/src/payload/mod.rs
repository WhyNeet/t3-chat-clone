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
