use crate::{
    data_models::session::SessionInput,
    routes::ApiResponse,
    util::user::{check_username::check_username, verify_session::verify_session},
    DB,
};
use axum::{extract::Json, http::StatusCode};
use tracing::error;

pub async fn post_verify_session(
    Json(payload): Json<SessionInput>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
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

    // Check if username is valid
    let validity_check = check_username(&payload.username);
    if !validity_check {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                is_error: true,
                error_msg: Some(String::from("The Username contains invalid characters.")),
                data: None,
            }),
        );
    }

    let verify_result = verify_session(&payload, pool).await;

    match verify_result {
        Ok(result) => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    is_error: false,
                    error_msg: None,
                    data: Some(result),
                }),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(format!("An error occurred: {:?}", e)),
                    data: None,
                }),
            );
        }
    }
}
