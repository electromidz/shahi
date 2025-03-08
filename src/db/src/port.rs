pub trait BlockchainDB: Send + Sync {
    fn create_account(&self, address: String, public_key: Vec<u8>);
}
