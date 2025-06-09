use redis_om::HashModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, HashModel, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}
