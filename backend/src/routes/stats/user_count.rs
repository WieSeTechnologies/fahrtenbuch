use crate::util::user::count::fetch_user_count;
use crate::DB;
use axum::http::StatusCode;

pub async fn get_user_count() -> (StatusCode, String) {
    let pool = DB.get();

    let count = fetch_user_count(match pool {
        Some(pool) => pool,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, String::from(""));
        }
    })
    .await;

    match count {
        Ok(count) => {
            return (StatusCode::OK, count.to_string());
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, String::from(""));
        }
    }
}
