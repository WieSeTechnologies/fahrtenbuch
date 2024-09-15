use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub uuid: uuid::Uuid,
    pub from_username: String,
    pub to_username: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub confirmation_date: chrono::DateTime<chrono::Utc>,
}
