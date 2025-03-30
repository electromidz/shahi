use rocksdb::Transaction;
use account::Account;

pub trait BlockchainDB: Send + Sync {
    fn create_account(&self, account:&Account);
    fn get_account(&self, address: String);
    fn get_balance(&self, address: String) -> Result<u64, String>;
    // fn add_transaction(&self, transaction: Transaction)-> Result<(), String>;
}
