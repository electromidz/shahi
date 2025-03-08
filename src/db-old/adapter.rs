use super::port::BlockchainDB;
use crate::block::Block;
use rocksdb::{DB, Options};

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
    fn create_account(&self, address: String, public_key: Vec<u8>) {}
}
