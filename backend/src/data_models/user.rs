use super::payment::Payment;
use super::session::Session;
use super::vehicle::Vehicle;

pub enum UserRole {
    Restricted,
    Normal,
    Admin,
}

pub struct User {
    username: String,
    displayname: String,
    password_hash: String,
    new_password_required: bool,
    sessions: Vec<Session>,
    vehicles: Vec<Vehicle>,
    payments: Vec<Payment>,
    role: UserRole,
}
