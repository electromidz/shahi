pub mod types;
use types::types::InitResult;

pub fn init() -> InitResult {
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_server() -> InitResult {
        let init_server = init()?; // Unwrap the Result
        assert_eq!(init_server, true); // Compare the inner bool value
        Ok(true) // Explicitly return `Ok(())`
    }
}
