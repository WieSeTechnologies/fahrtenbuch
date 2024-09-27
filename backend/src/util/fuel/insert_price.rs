use crate::{
    data_models::fuel::InsertFuelPrice, routes::AuthenticatedRequest,
    util::user::verify_session::verify_session,
};
use chrono::Local;
use sqlx::PgPool;

pub async fn insert_fuel_price(
    payload: AuthenticatedRequest<InsertFuelPrice>,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let session = payload.session;
    let data = payload.data;

    // Check if session is valid. If not, throw an error.
    let session_validity = match verify_session(&session, pool).await {
        Ok(validity) => validity,
        Err(_) => {
            return Err("Could not check validity of session.".into());
        }
    };
    if !session_validity {
        return Err("Invalid Session.".into());
    }

    // TODO: Finish this function by actually inserting stuff
    let now = Local::now();

    todo!();

    Ok(())
}
