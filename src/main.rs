use blockchain::Blockchain;
use mempool::Mempool;
use transaction::Transaction;
//use peer_to_peer::PeerToPeer;
use std::error::Error;

// modules
mod block;
mod blockchain;
mod mempool;
mod peer_to_peer;
mod transaction;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut blockchain = Blockchain::new(1);

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let sender = public_key.to_string();
    let receiver = "receiver_address".to_string();

    // Create a new transaction
    let amount = 100;
    let transaction = Transaction::new(sender, receiver, amount, &secret_key);

    let mut mempool = Mempool::new();
    let rans = mempool.get_transactions();

    println!("{:?}", rans);

    Ok(())
}

// println!("Mining block 1...");
// blockchain.add_block("First block data".to_string());
//
// println!("Mining block 2...");
// blockchain.add_block("Second block data".to_string());
//
// println!("Mining block 3...");
// blockchain.add_block("Third block data".to_string());
//
// println!("\nBlockchain:");
// for block in &blockchain.chain {
//     println!("{}", block);
// }
//
// println!("\nIs blockchain valid? {}", blockchain.is_chain_valid());
//
// PeerToPeer::init().await?;
