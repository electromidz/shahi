use rocksdb::{DB, Options};
pub mod port;

use port::BlockchainDB;

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
    fn create_account(&self, address: String, public_key: Vec<u8>) {
        let key = format!("account:{}", address);
        let value = hex::encode(public_key); // Convert public key to a hex string

        match self.db.put(key.as_bytes(), value.as_bytes()) {
            Ok(_) => println!("Account {} created successfully!", address),
            Err(e) => eprintln!("Failed to create account {}: {:?}", address, e),
        }
    }

    fn get_account(&self, address: String) {
        let key = format!("account:{}", address);

        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                let decoded =
                    String::from_utf8(value).unwrap_or_else(|_| "Invalid UTF-8".to_string());
                println!("Account {}: {}", address, decoded);
            }
            Ok(None) => println!("Account {} not found.", address),
            Err(e) => eprintln!("Failed to retrieve account {}: {:?}", address, e),
        }
    }
}
