use axum::{http::StatusCode, Json};

use super::ApiResponse;

pub async fn status() -> (StatusCode, Json<ApiResponse<String>>) {
    (
        StatusCode::OK,
        Json(ApiResponse {
            is_error: false,
            error_msg: None,
            data: Some(String::from("The Backend Server is running.")),
        }),
    )
}
