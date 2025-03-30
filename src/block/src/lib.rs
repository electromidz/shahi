use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::fmt;
use tracing::info;

use transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let nonce = 0;
        let hash = Block::calculate_hash(index, &timestamp, &transactions, &previous_hash, nonce);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: &DateTime<Utc>,
        transactions: &Vec<Transaction>,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!(
            "{}{}{:?}{}{}",
            index, timestamp, transactions, previous_hash, nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = Block::calculate_hash(
                self.index,
                &self.timestamp,
                &self.transactions,
                &self.previous_hash,
                self.nonce,
            );
        }
        info!("Block mined: {}", self.hash);
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block::\n [\nindex: {}\n- timestamp: {}\n- Transactions: {:?}\n- previous_hash: {}\n, hash: {}\n- nonce: {}\n]",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.hash, self.nonce
        )
    }
}
