use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fuel_type", rename_all = "lowercase")]
pub enum FuelType {
    Gasoline,
    Diesel,
    Electric,
    Other,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "gasoline_type", rename_all = "lowercase")]
pub enum GasolineType {
    Super,
    SuperE10,
    SuperPlus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelPrice {
    id: i32,
    price: f32,
    date: chrono::DateTime<chrono::Utc>,
    fuel_type: FuelType,
    gasoline_type: Option<GasolineType>,
}

// ===== Utility Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertFuelPrice {
    price: f32,
    date: chrono::DateTime<chrono::Utc>,
    fuel_type: FuelType,
    gasoline_type: Option<GasolineType>,
}
