use serde::{Deserialize, Serialize};

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
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

// ===== Utility Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub displayname: String,
    pub password: String,
}
