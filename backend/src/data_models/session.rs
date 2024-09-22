use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub uuid: uuid::Uuid,
    pub expiry: chrono::DateTime<chrono::Local>,
    pub owner_username: String,
}

impl TryFrom<&PgRow> for Session {
    type Error = sqlx::Error;

    fn try_from(row: &PgRow) -> Result<Self, Self::Error> {
        let session: Session = Session {
            uuid: row.try_get("uuid")?,
            expiry: row.try_get("expiry")?,
            owner_username: row.try_get("owner_username")?,
        };

        Ok(session)
    }
}

// Helper Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInput {
    pub username: String,
    pub session_id: uuid::Uuid,
}
