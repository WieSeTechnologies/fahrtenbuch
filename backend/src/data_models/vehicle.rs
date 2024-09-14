use super::fuel::{FuelType, GasolineType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    uuid: String,
    name: String,
    brand: Option<String>,
    fuel_type: FuelType,
    gasoline_type: Option<GasolineType>,
    owner_username: String,
    fuel_consumption_per_100_km: f32,
    fuel_consumption_per_100_km_autobahn: f32,
    maintenance_cost_per_100_km: f32,
}
