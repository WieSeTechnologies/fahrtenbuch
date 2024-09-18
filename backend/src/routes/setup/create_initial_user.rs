use crate::data_models::user::CreateUser;
use crate::data_models::user::User;
use crate::data_models::user::UserRole;
use crate::util::password;
use crate::util::user::count::fetch_user_count;
use crate::util::user::insert::insert_user;
use crate::DB;
use axum::extract;
use axum::http::StatusCode;
use chrono::prelude::*;
use tracing::error;

/// Creates the initial admin User.
/// This function only runs if there are 0 registered users.
pub async fn create_initial_user(
    extract::Json(payload): extract::Json<CreateUser>,
) -> (StatusCode, String) {
    // Get the Database Pool
    let pool = match DB.get() {
        Some(pool) => pool,
        None => {
            error!("Could not get database connection.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not get database connection."),
            );
        }
    };

    // Check if users Table is empty
    let user_count = match fetch_user_count(pool).await {
        Ok(user_count) => user_count,
        Err(e) => {
            error!("Could not get user count: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not get user count."),
            );
        }
    };
    if user_count > 0 {
        error!("There are already registered users.");
        return (
            StatusCode::FORBIDDEN,
            String::from("There are already registered users."),
        );
    }

    // Hash the password
    let hashed_password = match password::hash_password(payload.password.as_str()) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Could not hash password: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not hash password."),
            );
        }
    };

    // Create the new User
    let user = User {
        username: payload.username,
        displayname: payload.displayname,
        password_hash: hashed_password,
        new_password_required: false,
        creation_date: Local::now(),
        role: UserRole::Admin,
    };

    if let Err(e) = insert_user(&user, pool).await {
        error!("Could not create user: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not create user: {:?}", e),
        );
    };

    (
        StatusCode::CREATED,
        String::from("Sucessfully created user."),
    )
}
