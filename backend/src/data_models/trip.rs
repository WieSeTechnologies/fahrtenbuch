use super::{route::Route, trip_template::TripTemplate, user::User, vehicle::Vehicle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TripType {
    Template(TripTemplate),
    Custom(Route),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PayingPassengerPaymentAdjustment {
    None,
    Tip(f32),
    RoundUpTo(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayingPassenger {
    pub user: User,
    pub payment_adjustment: PayingPassengerPaymentAdjustment,
    pub distance_traveled_percent: f32,
}

// Vehicle Owner must be in either paying_passengers or free_passengers. Do not make them pay by default!
// It is assumed that the vehicle is driven by its owner, who is therefore a passenger of some kind.
#[derive(Debug, Serialize, Deserialize)]
pub struct Trip {
    pub trip_type: TripType,
    pub vehicle: Vehicle,
    pub paying_passengers: Vec<PayingPassenger>,
    pub free_passengers: Vec<User>,
}
