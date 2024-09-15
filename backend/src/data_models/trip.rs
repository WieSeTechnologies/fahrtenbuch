use super::trip_route::TripRoute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trip_type", rename_all = "lowercase")]
pub enum TripType {
    Template,
    Custom,
}

// Vehicle Owner must be in either paying_passengers or free_passengers. Do not make them pay by default!
// It is assumed that the vehicle is driven by its owner, who is therefore a passenger of some kind.
#[derive(Debug, Serialize, Deserialize)]
pub struct Trip {
    pub id: i32,
    pub trip_type: TripType,
    pub trip_template: Option<String>,
    pub trip_route: TripRoute,
    pub vehicle: uuid::Uuid,
}
