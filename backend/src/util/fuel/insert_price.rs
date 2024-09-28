use crate::data_models::fuel::{FuelPrice, InsertFuelPrice};
use chrono::Local;
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub async fn insert_fuel_price(
    payload: InsertFuelPrice,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Finish this function by actually inserting stuff
    let now = Local::now();

    let query_result = sqlx::query(
                "INSERT INTO fuel_prices (price, date, fuel_type, gasoline_type) VALUES ($1, $2, $3, $4) RETURNING *",
            )
            .bind(payload.price)
            .bind(now)
            .bind(payload.fuel_type)
            .bind(payload.gasoline_type)
            .fetch_one(pool)
            .await?;

    let fuel_price: FuelPrice = FuelPrice::try_from(&query_result)?;

    dbg!(&fuel_price);

    todo!();

    Ok(())
}
