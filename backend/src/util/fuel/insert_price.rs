use crate::data_models::fuel::{FuelPrice, FuelType, InsertFuelPrice};
use chrono::Local;
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub async fn insert_fuel_price(
    payload: InsertFuelPrice,
    pool: &PgPool,
) -> Result<FuelPrice, Box<dyn std::error::Error>> {
    let now = Local::now();

    // Gasoline types for non-gasoline fuels are not allowed
    if payload.fuel_type != FuelType::Gasoline {
        return Err("There is no gasoline type for the selected fuel type.".into());
    }

    // Gasolines must have a gasoline type
    if payload.fuel_type == FuelType::Gasoline && payload.gasoline_type == None {
        return Err("Gasolines must have a fuel type specified.".into());
    }

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

    debug!("Fuel price created.");

    Ok(fuel_price)
}
