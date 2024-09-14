use super::user::User;

pub struct Payment {
    from: User,
    to: User,
    // TODO: Change String to propper Datetime Variable
    date: String,
    // TODO: Change String to propper Datetime Variable
    confirmation_date: String,
}
