use super::check_username::check_username;
use crate::data_models::session::{InvalidateAllSessions, Session};
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

// /// Invalidates all active sessions and
// pub async fn invalidate_session(
//     verify_session: &InvalidateAllSessions,
//     pool: &PgPool,
// ) -> Result<bool, Box<dyn std::error::Error>> {
//     // Check username validity
//     if !check_username(&verify_session.username) {
//         let error_msg = "Invalid username.";
//         error!(error_msg);
//         return Err(error_msg.into());
//     }
// }
