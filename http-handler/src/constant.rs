#[derive(Debug)]
pub enum Woo2Err {
    ErrorInvalidSession,
    ErrorUserNotFound,
    ErrorNoVehiclesFound,
    ErrorDbAddValue,
    ErrorDbFetch,
}

impl Woo2Err {
    // Method to get the constant string value
    pub fn as_str(&self) -> &'static str {
        match self {
            Woo2Err::ErrorInvalidSession => "Session is invalid.",
            Woo2Err::ErrorUserNotFound => "User not found.",
            Woo2Err::ErrorNoVehiclesFound => "No vehicles found for user.",
            Woo2Err::ErrorDbAddValue => "Error registering value.",
            Woo2Err::ErrorDbFetch => "Error fetching database value.",
        }
    }
}
