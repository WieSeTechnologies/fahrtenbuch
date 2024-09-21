use crate::data_models::user::LoginUser;
use crate::routes::ApiResponse;
use crate::util::user::count::fetch_user_count;
use crate::util::user::login::login;
use crate::DB;
use axum::extract::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

/// Creates the initial admin User.
/// This function only runs if there are 0 registered users.
pub async fn get_session(
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
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

    if user_count == 0 {
        error!("There are no registered users.");
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                is_error: true,
                error_msg: Some(String::from("There are no registered users.")),
                data: None,
            }),
        );
    }

    // Try to login user
    match login(&payload, &pool).await {
        Ok(session) => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    is_error: false,
                    error_msg: None,
                    data: Some({
                        LoginResponse {
                            session_id: session.uuid.to_string(),
                        }
                    }),
                }),
            )
        }
        Err(e) => {
            error!("Login Error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(String::from("Something went wrong.")),
                    data: None,
                }),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub session_id: String,
}
