use std::fmt;

use bson::{Bson, doc};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: Vec<ChatMessageContent>,
    pub reasoning: Option<String>,
    pub role: Role,
    pub chat_id: ObjectId,
    pub model: Option<String>,
    pub updated_memory: Option<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: chrono::DateTime<Utc>,
}

impl Into<Bson> for ChatMessageContent {
    fn into(self) -> Bson {
        match self {
            ChatMessageContent::Text { value } => {
                Bson::Document(doc! { "type": "Text", "value": value })
            }
            ChatMessageContent::Image { id } => {
                Bson::Document(doc! { "type": "ImageUrl", "value": id })
            }
            ChatMessageContent::Pdf { id } => Bson::Document(doc! { "type": "Pdf", "id": id }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(tag = "type")]
pub enum ChatMessageContent {
    Text { value: String },
    Image { id: ObjectId },
    Pdf { id: ObjectId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum Role {
    #[default]
    User,
    Assistant,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::User => "user",
                Role::Assistant => "assistant",
            }
        )
    }
}
