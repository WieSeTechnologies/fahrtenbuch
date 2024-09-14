use super::user::User;

pub struct Payment {
    uuid: String,
    from_username: String,
    to_username: String,
    // TODO: Change String to propper Datetime Variable
    date: String,
    // TODO: Change String to propper Datetime Variable
    confirmation_date: Option<String>,
}
