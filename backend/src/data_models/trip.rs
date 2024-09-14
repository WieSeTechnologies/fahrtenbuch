use super::{route::Route, trip_template::TripTemplate, user::User, vehicle::Vehicle};

pub enum TripType {
    Template(TripTemplate),
    Custom(Route),
}

pub enum PayingPassengerPaymentAdjustment {
    None,
    Tip(f32),
    RoundUpTo(f32),
}

pub struct PayingPassenger {
    user: User,
    payment_adjustment: PayingPassengerPaymentAdjustment,
    distance_traveled_percent: f32,
}

// Vehicle Owner must be in either paying_passengers or free_passengers. Do not make them pay by default!
// It is assumed that the vehicle is driven by its owner, who is therefore a passenger of some kind.
pub struct Trip {
    trip_type: TripType,
    vehicle: Vehicle,
    paying_passengers: Vec<PayingPassenger>,
    free_passengers: Vec<User>,
}
