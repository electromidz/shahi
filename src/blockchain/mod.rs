use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap().clone();
        let mut new_block = Block::new(previous_block.index + 1, transactions, previous_block.hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the current block's hash is valid
            if current_block.hash
                != Block::calculate_hash(
                    current_block.index,
                    &current_block.timestamp,
                    &current_block.transactions,
                    &current_block.previous_hash,
                    current_block.nonce,
                )
            {
                println!("Invalid hash for block {}", current_block.index);
                return false;
            }

            // Check if the previous hash matches
            if current_block.previous_hash != previous_block.hash {
                println!("Invalid previous hash for block {}", current_block.index);
                return false;
            }
        }
        true
    }
}
