use libp2p::futures::StreamExt;
use libp2p::{gossipsub, Multiaddr, swarm::SwarmEvent};
use networks::Network;
use server::Server;
use std::error::Error;
use std::time::Duration;
use tokio::{time::sleep , io, io::AsyncBufReadExt};
use tracing::{error, info};

use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;

mod account;
pub mod block;
pub mod blockchain;
pub mod contracts;
pub mod mempool;
pub mod transaction;

use blockchain::Blockchain;
use mempool::Mempool;
use transaction::Transaction;
use gossipsub::Behaviour;

use libp2p::mdns;
use networks::libp2p::MyBehaviourEvent as GossipEvent;


#[derive(Debug)]
pub enum MyBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🦀");
    // Initialize logging
    tracing_subscriber::fmt::init();

    //Start the server in the background
    let server_handle = tokio::spawn(async {
        if let Err(e) = Server::start_server().await {
            error!("Server error: {:?}", e);
        }
    });

    //Main program continues executing other tasks
    info!("Server is running in the background...");

    // Simulate other work in the main program
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    info!("Main program is doing other work...");

    // Use `tokio::select!` to run both tasks concurrently
    tokio::select! {
        // Wait for the server task to finish (optional)
        _ = server_handle => {
            info!("Server task finished.");
        }
        // Simulate other work in the main program
        _ = async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            info!("Main program is doing other work...");

            // Simulate more work
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            info!("Main program finished its work.");
        } => {}
    }

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
        info!("✅ Transaction is valid. Adding to mempool...");
        mempool.add_transaction(transaction.clone());
    } else {
        info!("❌ Transaction verification failed.");
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
        info!("⚠️ No transactions available for the new block.");
    }

    // Print the current blockchain state
    info!("📌 Blockchain State:\n{:?}", blockchain);

    // Print the current mempool state
    info!("📌 Mempool State:\n{:?}", mempool.get_transactions());

    //server run

    // if let Err(e) = init_p_2_p().await {
    //     eprintln!("have error on init_p_2_p {:?}", e)
    // }
    //let mut network1 = Network::create().await;
    let mut network2 = Network::create().await;


    //Start listening on network1
    // network1
    //     .listen_on("/ip4/0.0.0.0/tcp/8080".parse().unwrap())
    //     .unwrap();

    info!("💈 Network1 is listening on /ip4/193.151.152.51/tcp/8080\n");

    // Give some time for network1 to start before dialing
    sleep(Duration::from_secs(2)).await;

    let topic = gossipsub::IdentTopic::new("test-net");
    let mut gossip = Network::create_gossip().await?;
    gossip.behaviour_mut().gossipsub.subscribe(&topic)?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Listen on all interfaces and whatever port the OS assigns
    gossip.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    gossip.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    info!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");




    // Network2 dials network1
    //match network2.dial("/ip4/193.151.152.51/tcp/8080".parse::<Multiaddr>().unwrap()) {
    match network2.dial("/ip4/127.0.0.1/tcp/8080".parse::<Multiaddr>().unwrap()) {
        Ok(_) => info!("📞 Network2 dialing Network1..."),
        Err(e) => {
            error!("❌ Network2 failed to dial: {:?}", e);
        }
    }
    //
    // Process events concurrently for both networks
    loop {
        tokio::select! {
            // event = network1.next() => {
            //     if let Some(event) = event {
            //         println!("🌐 Network1 Event: {:?}\n", event);
            //     }
            // }
            event = network2.next() => {
                if let Some(event) = event {
                    info!("📡 Network2 Event: {:?}\n", event);
                }
            }
            event = gossip.select_next_some() => match event {
                  SwarmEvent::Behaviour(GossipEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discovered a new peer: {peer_id}");
                        gossip.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(GossipEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discover peer has expired: {peer_id}");
                        gossip.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(GossipEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => println!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
    Ok(())
}
