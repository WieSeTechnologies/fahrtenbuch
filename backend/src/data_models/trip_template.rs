use super::trip_route::TripRoute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TripTemplate {
    pub id: i32,
    pub from_loc_name: String,
    pub to_loc_name: String,
    pub trip_route: TripRoute,
}
