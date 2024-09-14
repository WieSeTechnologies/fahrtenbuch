use axum::http::StatusCode;

pub async fn status() -> (StatusCode, String) {
    (
        StatusCode::OK,
        String::from("The backend server is running."),
    )
}
