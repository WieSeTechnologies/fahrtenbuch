use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub uuid: uuid::Uuid,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub owner_username: String,
}
