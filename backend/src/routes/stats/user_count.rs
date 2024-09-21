use crate::routes::ApiResponse;
use crate::util::user::count::fetch_user_count;
use crate::DB;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn get_user_count() -> (StatusCode, Json<ApiResponse<UserCountRespone>>) {
    let pool = DB.get();

    let count = fetch_user_count(match pool {
        Some(pool) => pool,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: None,
                    data: None,
                }),
            );
        }
    })
    .await;

    match count {
        Ok(count) => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    is_error: false,
                    error_msg: None,
                    data: Some(UserCountRespone { count: count }),
                }),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: None,
                    data: None,
                }),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCountRespone {
    pub count: i64,
}
