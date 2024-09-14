use serde::{Deserialize, Serialize};
use super::route::Route;

#[derive(Debug, Serialize, Deserialize)]
pub struct TripTemplate {
    pub from: String,
    pub to: String,
    pub route: Route,
}
