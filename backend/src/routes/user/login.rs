use crate::data_models::user::LoginUser;
use crate::util::user::count::fetch_user_count;
use crate::util::user::login::login;
use crate::DB;
use axum::extract;
use axum::http::StatusCode;
use tracing::error;

/// Creates the initial admin User.
/// This function only runs if there are 0 registered users.
pub async fn get_session(extract::Json(payload): extract::Json<LoginUser>) -> (StatusCode, String) {
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

    if user_count == 0 {
        error!("There are no registered users.");
        return (
            StatusCode::FORBIDDEN,
            String::from("There are no registered users."),
        );
    }

    // Try to login user
    match login(&payload, &pool).await {
        Ok(session) => return (StatusCode::OK, String::from(session.uuid)),
        Err(e) => {
            error!("Login Error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Something went wrong."),
            );
        }
    }
}
