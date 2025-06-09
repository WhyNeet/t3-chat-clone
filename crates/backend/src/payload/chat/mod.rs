use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatPayload {
    pub id: String,
    pub name: Option<String>,
    pub user_id: String,
    pub timestamp: chrono::DateTime<Utc>,
}
