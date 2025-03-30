use transaction::Transaction;
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

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn remove_transaction(&mut self, transaction: &Transaction) {
        if let Some(pos) = self.transactions.iter().position(|t| t == transaction) {
            self.transactions.remove(pos);
        }
    }
}
