use crate::block::Block;
pub struct ProotOfWork {
    pub difficulty: usize,
}

impl ProotOfWork {
    pub fn new(difficulty: usize) -> Self {
        ProotOfWork { difficulty }
    }

    pub fn mine_block(&self, block: &mut Block) {
        let prefix = "0".repeat(self.difficulty);
        while !block.hash.starts_with(&prefix) {
            block.nonce += 1;
            block.hash = Block::calculate_hash(
                block.index,
                &block.timestamp,
                &block.transactions,
                &block.previous_hash,
                block.nonce,
            );
        }
    }
}
