use crate::data_models::session::SessionInput;
use crate::util::user::check_username::check_username;
use crate::DB;
use crate::{routes::ApiResponse, util::user::invalidate_session::invalidate_session};
use axum::{http::StatusCode, Json};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub async fn post_invalidate_session(
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

    match invalidate_session(&payload, pool).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    is_error: false,
                    error_msg: None,
                    data: Some(true),
                }),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(format!(
                        "An Error occurred during the invalidation: {:?}",
                        e
                    )),
                    data: None,
                }),
            );
        }
    };
}
