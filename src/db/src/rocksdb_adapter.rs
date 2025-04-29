use rocksdb::{DB, Options};
use tracing::{error, info};
use account::Account;
use bincode::{encode_to_vec, config};

use transaction::Transaction as TransactionBalance;
use crate::port::BlockchainDB;

pub struct RocksDBAdapter {
    db: DB,
}

impl RocksDBAdapter {
    pub fn new(path: &str) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }
}

impl BlockchainDB for RocksDBAdapter {
    fn create_account(&self, account: &Account) {
        let key = format!("account:{}", account.address);

        // Serialize the account struct to a binary format using Bincode 2
        match encode_to_vec(&account, config::standard()) {
            Ok(encoded) => {
                // Store the serialized data in RocksDB
                if let Err(e) = self.db.put(key.as_bytes(), &encoded) {
                    error!("DB::Failed to create account {}: {:?}", account.address, e);
                } else {
                    info!("DB::Account {} created successfully!", account.address);
                }
            }
            Err(e) => error!("DB::Serialization failed for account {}: {:?}", account.address, e),
        }
    }

    fn get_account(&self, address: String) {
        let key = format!("account:{}", address);

        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                let decoded =
                    String::from_utf8(value).unwrap_or_else(|_| "Invalid UTF-8".to_string());
                info!("DB::Account {}: {}", address, decoded);
            }
            Ok(None) => info!("DB::Account {} not found.", address),
            Err(e) => error!("DB::Failed to retrieve account {}: {:?}", address, e),
        }
    }

    fn get_balance(&self, address: String) -> Result<u64, String> {
        let key = format!("account:{}", address);
        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                // Deserialize the account data using Bincode
                match bincode::decode_from_slice::<Account, _>(&value, config::standard()) {
                    Ok((account, _)) => {
                        info!("DB::Account found. Address: {}, Account: {:?}", account.address, account);
                        Ok(account.balance)
                    }
                    Err(_) => {
                        error!("DB::Failed to deserialize account data for address: {}", address);
                        Err("Failed to deserialize account data".to_string())
                    }
                }
            }
            Ok(None) => {
                info!("DB::Account not found for address: {}", address);
                Err("Account not found".to_string())
            }
            Err(e) => {
                error!("DB::Failed to retrieve account {}: {:?}", address, e);
                Err(format!("Database error: {:?}", e))
            }
        }
    }

    #[warn(unused_variables)]
    fn add_transaction(&self, _transaction: &TransactionBalance) -> Result<(), String> {
        Ok(())
    }
}
