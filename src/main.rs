use libp2p::futures::StreamExt;
use libp2p::{gossipsub, Multiaddr, swarm::SwarmEvent};
use networks::Network;
use server::Server;
use std::error::Error;
use std::time::Duration;
use tokio::{time::sleep , io, io::AsyncBufReadExt};
use tracing::{error, info};

use libp2p::mdns;
use networks::libp2p::{MyBehaviourEvent as GossipEvent};


#[derive(Debug)]
pub enum MyBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
}
#[tokio::main]
//async fn main() -> Result<(), Box<dyn Error>> {
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

    // Use `tokio::select!` to run both tasks concurrently
    tokio::select! {
        // Wait for the server task to finish (optional)
        _ = server_handle => {
            info!("Server task finished.");
        }
        // Simulate other work in the main program
        _ = async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            info!("Main program is doing other work...");

            // Simulate more work
            tokio::time::sleep(Duration::from_secs(2)).await;
            info!("Main program finished its work.");
        } => {}
    }

    // Give some time for network1 to start before dialing
    sleep(Duration::from_secs(1)).await;
    match Network::start_gossip().await {
        Ok(_)=> info!("gossip start"),
        Err(e)=> error!("gossip start: {}", e)
    }

    let mut swarm = match Network::create_gossip().await {
        Ok(mut swarm) => {
            let topic = gossipsub::IdentTopic::new("test-net");
            swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();
            swarm
        }
        Err(e)=>{error!("❌ Failed to create gossipsub swarm: {:?}", e); return Err(e);},
    };
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let topic = gossipsub::IdentTopic::new("test-net");

    let mut net_dial_1 = Network::create().await;
    let mut net_listen_1 = Network::create().await;

    let dial_addr = "/ip4/127.0.0.1/tcp/8080".parse::<Multiaddr>().unwrap();
    let listen_addr = "/ip4/127.0.0.1/tcp/8080".parse::<Multiaddr>().unwrap();

    match Network::dial(&mut net_dial_1, dial_addr).await {
        Ok(_) => info!("✅ Dial successful"),
        Err(e) => {
            error!("❌ Dial error: {}",e);
            return Err(Box::new(e));
        },
    };

    match Network::listen(&mut net_listen_1, listen_addr).await {
        Ok(_) => info!("✅ Listen successful"),
        Err(e) => error!("❌ Listen error: {}", e),
    }

    loop {
        tokio::select! {
            event = net_dial_1.next() => {
                if let Some(event) = event {
                    info!("📡 Network1 Event: {:?}", event);
                }
            }
            event = net_listen_1.next() => {
                if let Some(event) = event {
                    info!("🌐 Network1 Event: {:?}", event);
                }
            }
            line = stdin.next_line() => {
                    if let Ok(Some(input)) = line {
                        let message = input.clone();
                        info!("✉️ Sending message: {}", message);
                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.as_bytes()) {
                            error!("❌ Failed to send message: {:?}", e);
                        } else {
                            info!("📨 Message sent!");
                        }
                    }
                }

                event = swarm.select_next_some() => match event {
                  SwarmEvent::Behaviour(GossipEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        info!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(GossipEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        info!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(GossipEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => info!(
                        "📩 Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
}
