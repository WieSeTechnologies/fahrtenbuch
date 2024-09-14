use axum::http::StatusCode;

/// Creates the initial admin User.
/// This function only runs if there are 0 registered users.
pub async fn create_initial_user() -> (StatusCode, String) {
    (
        StatusCode::NOT_IMPLEMENTED,
        String::from("This feature is not yet implemented."),
    )
}
