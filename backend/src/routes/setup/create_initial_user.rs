use crate::data_models::user::CreateUser;
use crate::data_models::user::User;
use crate::data_models::user::UserRole;
use crate::routes::ApiResponse;
use crate::util::password;
use crate::util::user::count::fetch_user_count;
use crate::util::user::insert::insert_user;
use crate::DB;
use axum::extract;
use axum::http::StatusCode;
use axum::Json;
use chrono::prelude::*;
use tracing::error;

/// Creates the initial admin User.
/// This function only runs if there are 0 registered users.
pub async fn create_initial_user(
    extract::Json(payload): extract::Json<CreateUser>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    // Get the Database Pool
    let pool = match DB.get() {
        Some(pool) => pool,
        None => {
            error!("Could not get database connection.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(String::from("Could not get database connection.")),
                    data: None,
                }),
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
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(String::from("Could not get user count.")),
                    data: None,
                }),
            );
        }
    };
    if user_count > 0 {
        error!("There are already registered users.");
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                is_error: true,
                error_msg: Some(String::from("There are already registered users.")),
                data: None,
            }),
        );
    }

    // Hash the password
    let hashed_password = match password::hash_password(payload.password.as_str()) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Could not hash password: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(String::from("Could not hash password.")),
                    data: None,
                }),
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
            Json(ApiResponse {
                is_error: true,
                error_msg: Some(format!("Could not create user: {:?}", e)),
                data: None,
            }),
        );
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse {
            is_error: false,
            error_msg: None,
            data: Some(String::from("Sucessfully created user.")),
        }),
    )
}
