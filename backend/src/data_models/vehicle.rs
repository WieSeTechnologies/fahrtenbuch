use super::{fuel::FuelType, user::User};

pub struct Vehicle {
    uuid: String,
    name: String,
    brand: Option<String>,
    fuel: FuelType,
    owner: User,
    fuel_consumption_per_100_km: f32,
    fuel_consumption_per_100_km_autobahn: f32,
    maintenance_cost_per_100_km: f32,
}
