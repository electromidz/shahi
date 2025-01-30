use crate::transaction::Transaction;
use std::collections::VecDeque;

pub struct Mempool {
    pub transactions: VecDeque<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: VecDeque::new(),
        }
    }

    // Add a transaction to the mempool
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push_back(transaction);
    }

    // Get all transactions from the mempool (for mining a new block)
    pub fn get_transactions(&mut self) -> Vec<Transaction> {
        self.transactions.drain(..).collect()
    }
}
