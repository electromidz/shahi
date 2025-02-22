use blockchain::Blockchain;

use libp2p::futures::StreamExt;
use libp2p::Multiaddr;
use mempool::Mempool;
use networks::Network;
use server::Server;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
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
    println!("🦀");
    // Initialize logging
    tracing_subscriber::fmt::init();
    Server::start_server().await;

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

    //server run

    // if let Err(e) = init_p_2_p().await {
    //     eprintln!("have error on init_p_2_p {:?}", e)
    // }
    //let mut network1 = Network::create().await;
    // let mut network2 = Network::create().await;
    //
    // //Start listening on network1
    // // network1
    // //     .listen_on("/ip4/0.0.0.0/tcp/8080".parse().unwrap())
    // //     .unwrap();
    //
    // println!("💈 Network1 is listening on /ip4/0.0.0.0/tcp/8080\n");
    //
    // // Give some time for network1 to start before dialing
    // sleep(Duration::from_secs(2)).await;
    //
    // // Network2 dials network1
    // match network2.dial("/ip4/127.0.0.1/tcp/8080".parse::<Multiaddr>().unwrap()) {
    //     Ok(_) => println!("📞 Network2 dialing Network1..."),
    //     Err(e) => {
    //         eprintln!("❌ Network2 failed to dial: {:?}", e);
    //     }
    // }
    // //
    // // Process events concurrently for both networks
    // loop {
    //     tokio::select! {
    //         // event = network1.next() => {
    //         //     if let Some(event) = event {
    //         //         println!("🌐 Network1 Event: {:?}\n", event);
    //         //     }
    //         // }
    //         event = network2.next() => {
    //             if let Some(event) = event {
    //                 println!("📡 Network2 Event: {:?}\n", event);
    //             }
    //         }
    //     }
    // }
    Ok(())
}
