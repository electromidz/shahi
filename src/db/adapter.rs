use rocksdb::{Options, DB};

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
    fn save_block(&self, block: Block) -> Result<(), DBError> {
        // Serialize the block and save it to RocksDB
        let serialized_block = serde_json::to_vec(&block)?;
        self.db.put(block.hash(), serialized_block)?;
        Ok(())
    }

    fn get_block(&self, block_hash: &str) -> Result<Option<Block>, DBError> {
        // Retrieve the block from RocksDB
        if let Some(serialized_block) = self.db.get(block_hash)? {
            let block = serde_json::from_slice(&serialized_block)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    // Implement other methods...
}
