// src/persistent_state/rocksdb.rs
use rocksdb::{DB, Options};
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub address: String,
    pub public_key: Vec<u8>,
    pub balance: u64,
    pub sequence_number: u64,
}

pub struct PersistentState {
    db: DB,
}

impl PersistentState {
    /// Create a new PersistentState instance with RocksDB.
    pub fn new(path: &str) -> Self {
        let db = DB::open_default(Path::new(path)).unwrap();
        PersistentState { db }
    }

    /// Create a new account and store it in RocksDB.
    pub fn create_account(&self, address: String, public_key: Vec<u8>) {
        let account = Account {
            address: address.clone(),
            public_key,
            balance: 0,
            sequence_number: 0,
        };
        let serialized = bincode::serialize(&account).unwrap();
        self.db.put(address.as_bytes(), serialized).unwrap();
    }

    /// Retrieve an account from RocksDB by address.
    pub fn get_account(&self, address: &str) -> Option<Account> {
        if let Ok(Some(serialized)) = self.db.get(address.as_bytes()) {
            let account: Account = bincode::deserialize(&serialized).unwrap();
            Some(account)
        } else {
            None
        }
    }

    /// Update an account's balance in RocksDB.
    pub fn update_balance(&self, address: &str, new_balance: u64) -> Result<(), String> {
        if let Some(mut account) = self.get_account(address) {
            account.balance = new_balance;
            let serialized = bincode::serialize(&account).unwrap();
            self.db.put(address.as_bytes(), serialized).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Increment the sequence number (nonce) for an account.
    pub fn increment_sequence_number(&self, address: &str) -> Result<(), String> {
        if let Some(mut account) = self.get_account(address) {
            account.sequence_number += 1;
            let serialized = bincode::serialize(&account).unwrap();
            self.db.put(address.as_bytes(), serialized).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }
}