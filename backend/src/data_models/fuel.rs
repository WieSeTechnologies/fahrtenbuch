pub enum FuelType {
    Gasoline,
    Diesel,
    Electric,
    Other,
}

pub enum GasolineType {
    Super,
    SuperE10,
    SuperPlus,
}

pub struct FuelPrice {
    id: i32,
    price: f32,
    // TODO: Change String to propper Datetime Variable
    date: String,
}
