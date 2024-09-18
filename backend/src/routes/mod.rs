pub mod setup;
pub mod stats;
pub mod status;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub is_error: bool,
    pub error_msg: Option<String>,
    pub data: Option<T>,
}
