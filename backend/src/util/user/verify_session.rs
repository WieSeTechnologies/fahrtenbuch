use super::check_username::check_username;
use crate::data_models::session::{Session, VerifySession};
use chrono::{DateTime, Local};
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub async fn verify_session(
    verify_session: &VerifySession,
    pool: &PgPool,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Check username validity
    if !check_username(&verify_session.username) {
        let error_msg = "Invalid username.";
        error!(error_msg);
        return Err(error_msg.into());
    }

    // Get the session
    let query_result = sqlx::query("SELECT * FROM sessions WHERE uuid = $1")
        .bind(&verify_session.session_id)
        .fetch_all(pool)
        .await?;

    // Check the result count and get the session
    if &query_result.len() < &1_usize {
        return Ok(false);
    }

    let row = match query_result.first() {
        Some(result) => result,
        None => {
            error!("There are no results.");
            return Err("There are no results.".into());
        }
    };

    let session: Session = Session::try_from(row)?;

    // Check if the session is still valid
    if session.expiry < Local::now() {
        info!("Session is no longer valid.");
        return Ok(false);
    }

    // Check if the usernames match
    if verify_session.username != session.owner_username {
        info!("Usernames do not match.");
        return Ok(false);
    }

    // Session is valid
    Ok(true)
}
