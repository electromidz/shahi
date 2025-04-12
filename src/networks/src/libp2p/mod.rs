pub use libp2p::swarm::dummy::Behaviour as DummyBehaviour;
pub use libp2p::{
    futures::{ StreamExt},
    gossipsub, mdns, noise, ping,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, SwarmBuilder,
};

use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};
use libp2p::gossipsub::IdentTopic;
use tracing::{info, error, warn};
use tokio::io;
pub use libp2p::swarm::DialError;


pub struct Libp2pNetwork {
    swarm: Swarm<ping::Behaviour>,
}

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub enum MyBehaviourEvents {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
}

impl MyBehaviour {
    pub async fn create_gossip_swarm() -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
        let tcp_config = tcp::Config::default();
        let security_upgrade = noise::Config::new;
        let multiplexer_upgrade = yamux::Config::default;
        let swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(tcp_config, security_upgrade, multiplexer_upgrade)?
            .with_quic()
            .with_behaviour(|key| {
                // To content-address message, we can take the hash of message and use it as an ID.
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                // Set a custom gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                    .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message
                    // signing)
                    .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

                // build a gossipsub network behaviour
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns =
                    mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())
                        .expect("Failed to create mdns Behaviour");
                Ok(MyBehaviour { gossipsub, mdns })
            })?
            .build();
        Ok(swarm)
    }

    pub async fn start_gossip()->Result<Swarm<MyBehaviour>, Box<dyn Error>> {
        let topic = IdentTopic::new("test-net");
        let mut swarm = MyBehaviour::create_gossip_swarm().await?;
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        info!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");
        Ok(swarm)
    }
}

#[warn(dead_code)]
impl Libp2pNetwork {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let timeout = Duration::from_secs(u64::MAX);
        let tcp_config = tcp::Config::default();
        let security_upgrde = noise::Config::new;
        let multiplexer_upgrade = yamux::Config::default;
        let swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(tcp_config, security_upgrde, multiplexer_upgrade)?
            .with_behaviour(|_| ping::Behaviour::default())?
            .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(timeout))
            .build();

        Ok(Self { swarm })
    }

    pub async fn create_swarm() -> Swarm<DummyBehaviour> {
        let tcp_config = tcp::Config::default();
        let security_upgrade = noise::Config::new;
        let multiplexer_upgrade = yamux::Config::default;
        SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(tcp_config, security_upgrade, multiplexer_upgrade)
            .unwrap()
            .with_behaviour(|_| DummyBehaviour)
            .unwrap()
            .build()
    }

    /// Start listening on a given address
    pub async fn listen(&mut self, address:Multiaddr) -> Result<(), Box<dyn Error>> {
        let mut listen = Self::create_swarm().await;
        match listen.listen_on(address) {
            Ok(_) =>{
                info!("👂 Network2 listening on Network1...");
            },
            Err(e) => {
                error!("❌ Network2 failed to listen: {:?}", e);
            }
        }
        Ok(())
    }

    /// Dial a given address
    pub async fn dial(&mut self, address: Multiaddr) -> Result<(), DialError> {
        let mut dial = Self::create_swarm().await;
        match dial.dial(address) {
            Ok(_) =>{
                info!("📞 Network2 dialing Network1...");
            },
            Err(e) => {
                error!("❌ Network2 failed to dial: {:?}", e);
                return Err(e);
            }
        };
        tokio::spawn(async move {
            while let Some(event) = dial.next().await {
                info!("🌐 Network1 Event: {:?}", event);
            }
        });
        Ok(())
    }
}

#[test]
fn instance_swarm() {
    let network = Libp2pNetwork::new();
    assert!(network.is_ok(), "Faild to create network");
}