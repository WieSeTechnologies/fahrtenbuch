use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trip_route")]
pub struct TripRoute {
    pub distance: f32,
    pub distance_autobahn: f32,
}
