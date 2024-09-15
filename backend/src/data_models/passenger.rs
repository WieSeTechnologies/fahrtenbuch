use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(
    type_name = "paying_passenger_payment_adjustment_type",
    rename_all = "lowercase"
)]
pub enum PayingPassengerPaymentAdjustmentType {
    None,
    Tip,
    RoundUpTo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Passenger {
    pub id: i32,
    pub trip_id: i32,
    pub username: String,
    pub payment_adjustment: PayingPassengerPaymentAdjustmentType,
    // Either the Tip or the Amount to round to
    pub payment_adjustment_amount: Option<f32>,
    pub distance_traveled_percent: f32,
    pub is_paying: bool,
}
