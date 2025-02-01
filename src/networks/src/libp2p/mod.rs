pub mod types; // Declare the `types` module

use types::types::InitResult; // Import `InitResult` from the `types` module

pub struct Network {}

impl Network {
    pub fn init() -> InitResult {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_server() -> Result<(), Box<dyn std::error::Error>> {
        let init_server = Network::init()?; // Unwrap the Result
        assert_eq!(init_server, true); // Compare the inner bool value
        Ok(()) // Return `Ok(())` for tests
    }
}
