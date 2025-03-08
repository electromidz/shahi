pub trait BlockchainDB {
    fn create_account(&self, address: String, public_key: Vec<u8>);
    fn update_balance(&self, address: &str, new_balance: u64) -> Result<(), String>;
}
