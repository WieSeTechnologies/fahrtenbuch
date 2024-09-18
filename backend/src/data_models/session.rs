use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub uuid: uuid::Uuid,
    pub expiry: chrono::DateTime<chrono::Local>,
    pub owner_username: String,
}
