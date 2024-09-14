use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FuelType {
    Gasoline,
    Diesel,
    Electric,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GasolineType {
    Super,
    SuperE10,
    SuperPlus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelPrice {
    id: i32,
    price: f32,
    // TODO: Change String to propper Datetime Variable
    date: String,
}
