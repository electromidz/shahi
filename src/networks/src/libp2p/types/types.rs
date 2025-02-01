use std::error::Error;
// Define the `InitResult` type alias
pub type InitResult = Result<bool, Box<dyn Error>>;
