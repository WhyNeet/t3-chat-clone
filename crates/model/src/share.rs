use redis_om::HashModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, HashModel, Serialize, Deserialize, Clone)]
pub struct Share {
    pub id: String,
    pub share_id: String,
}
