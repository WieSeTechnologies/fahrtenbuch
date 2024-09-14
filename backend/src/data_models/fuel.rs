pub enum FuelType {
    Gasoline(GasolineType),
    Diesel,
    Electric,
    Other(String),
}

pub enum GasolineType {
    Super,
    SuperE10,
    SuperPlus,
    Other(String),
}

pub struct FuelPrice {
    price: f32,
    // TODO: Change String to propper Datetime Variable
    date: String,
}
