use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub uuid: String,
    pub from_username: String,
    pub to_username: String,
    // TODO: Change String to propper Datetime Variable
    pub date: String,
    // TODO: Change String to propper Datetime Variable
    pub confirmation_date: Option<String>,
}
