use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Restricted,
    Normal,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub displayname: String,
    pub password_hash: String,
    pub new_password_required: bool,
    pub role: UserRole,
    pub creation_date: chrono::DateTime<chrono::Local>,
}

impl TryFrom<&PgRow> for User {
    type Error = sqlx::Error;

    fn try_from(row: &PgRow) -> Result<Self, Self::Error> {
        let user = User {
            username: row.try_get("username")?,
            displayname: row.try_get("displayname")?,
            password_hash: row.try_get("password_hash")?,
            new_password_required: row.try_get("new_password_required")?,
            creation_date: row.try_get("creation_date")?,
            role: row.try_get("role")?,
        };

        Ok(user)
    }
}

// ===== Utility Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub displayname: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}
