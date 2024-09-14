use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub uuid: String,
    // TODO: Change String to propper Datetime Variable
    pub expiry: String,
    pub owner_username: String,
}
