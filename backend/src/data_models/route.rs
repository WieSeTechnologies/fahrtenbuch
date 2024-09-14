use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub distance: f32,
    pub distance_autobahn: f32,
}
