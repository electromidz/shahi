mod lib;

use blockchain::Blockchain;

use mempool::Mempool;
use networks::init_p_2_p;
use networks::libp2p::Libp2pNetwork;
use std::error::Error;
use transaction::Transaction;
// modules
pub mod block;
pub mod blockchain;
pub mod contracts;
pub mod mempool;
pub mod transaction;

use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut blockchain = Blockchain::new(1);
    let mut mempool = Mempool::new();

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let sender = public_key.to_string();
    let receiver = "receiver_address".to_string();
    let payload = Some("{\"type\":\"message\", \"content\":\"Hello, Blockchain!\"}".to_string()); // Example message payload

    // Create a new transaction
    let amount = Some(100);
    let transaction = Transaction::new(sender, receiver, amount, payload, &secret_key);

    if transaction.verify_signature() {
        println!("✅ Transaction is valid. Adding to mempool...");
        mempool.add_transaction(transaction.clone());
    } else {
        println!("❌ Transaction verification failed.");
        return Ok(());
    }

    let transactions = mempool.get_transactions();

    // Add transactions to a new block
    if !transactions.is_empty() {
        blockchain.add_block(transactions.clone());

        // Remove transactions from mempool after adding them to a block
        for tx in transactions {
            mempool.remove_transaction(&tx);
        }
    } else {
        println!("⚠️ No transactions available for the new block.");
    }

    // Print the current blockchain state
    println!("📌 Blockchain State:\n{:?}", blockchain);

    // Print the current mempool state
    println!("📌 Mempool State:\n{:?}", mempool.get_transactions());

    if let Err(e) = init_p_2_p().await {
        eprintln!("have error on init_p_2_p", e)
    }
    Ok(())
}
