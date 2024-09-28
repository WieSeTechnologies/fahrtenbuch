use crate::{
    data_models::fuel::InsertFuelPrice,
    routes::{ApiResponse, AuthenticatedRequest},
    util::{fuel::insert_price::insert_fuel_price, user::verify_session::verify_session},
    DB,
};
use axum::{http::StatusCode, Json};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub async fn post_insert_fuel_price(
    Json(payload): Json<AuthenticatedRequest<InsertFuelPrice>>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
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

    // Check if session is valid. If not, throw an error.
    let session = payload.session;
    let session_validity = match verify_session(&session, pool).await {
        Ok(validity) => validity,
        Err(_) => {
            error!("Could not check validity of session.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(String::from("Could not check validity of session.")),
                    data: None,
                }),
            );
        }
    };
    if !session_validity {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                is_error: true,
                error_msg: Some(String::from("Invalid session.")),
                data: None,
            }),
        );
    }

    let data = payload.data;

    let _ = insert_fuel_price(data, pool).await;

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse {
            is_error: false,
            error_msg: None,
            data: Some(false),
        }),
    )
}
