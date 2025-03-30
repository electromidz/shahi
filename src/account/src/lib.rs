use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use bincode::{Decode, Encode};

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Account {
    pub address: String,          // Unique account address
    pub public_key: Vec<u8>,      // Public key (for verification)
    pub balance: u64,             // Account balance
    pub sequence_number: u64,     // Nonce for replay protection
    pub info: String,             // Account metadata
    pub permissions: Vec<String>, // Permissions for the account
    // Add other metadata as needed (e.g., staking info, permissions)
}

pub struct State {
    pub accounts: HashMap<String, Account>, // Address -> Account
}

impl State {
    pub fn new() -> Self {
        State {
            accounts: HashMap::new(),
        }
    }

    /// Create a new account
    pub fn create_account(&mut self, address: String, public_key: Vec<u8>) {
        let account = Account {
            address: address.clone(),
            public_key,
            balance: 0,
            sequence_number: 0,
            info: "".to_string(),
            permissions: vec![],
        };
        self.accounts.insert(address, account);
    }

    /// Get an account by address
    pub fn get_account(&self, address: &str) -> Option<&Account> {
        self.accounts.get(address)
    }

    /// Update an account's balance
    pub fn update_balance(&mut self, address: &str, new_balance: u64) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            account.balance = new_balance;
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    pub fn get_balance(&mut self, address: &str) -> Result<u64, String> {
        self.accounts
            .get(address)
            .map(|account| account.balance)
            .ok_or_else(|| "Account not found".to_string())
    }

    /// Increment the sequence number (nonce) for an account
    pub fn increment_sequence_number(&mut self, address: &str) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            account.sequence_number += 1;
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }
}
