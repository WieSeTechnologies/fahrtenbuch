use super::{check_username::check_username, verify_session::verify_session};
use crate::data_models::session::SessionInput;
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

/// Invalidates the session that calles the functions
pub async fn invalidate_session(
    session_input: &SessionInput,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check username validity
    if !check_username(&session_input.username) {
        let error_msg = "Invalid username.";
        error!(error_msg);
        return Err(error_msg.into());
    }

    // Check if current session is valid
    if !verify_session(session_input, pool).await? {
        return Err("Current session is invalid.".into());
    }

    // Invalidate the sessions
    sqlx::query("DELETE FROM sessions WHERE owner_username = $1 AND uuid = $2")
        .bind(&session_input.username)
        .bind(&session_input.session_id)
        .execute(pool)
        .await?;

    Ok(())
}
