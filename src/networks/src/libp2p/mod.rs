pub use libp2p::swarm::dummy::Behaviour as DummyBehaviour;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{
    futures::{io, StreamExt},
    gossipsub, mdns, noise, ping, quic,
    swarm::Swarm,
    tcp, yamux, Multiaddr, SwarmBuilder,
};

use std::{error::Error, time::Duration};
use tracing::info;

#[warn(dead_code)]
pub struct Libp2pNetwork {
    swarm: Swarm<ping::Behaviour>,
}

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
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

    pub async fn create_gossip() -> Swarm<MyBehaviour> {
        let tcp_config = tcp::Config::default();
        let security_upgrade = noise::Config::new;
        let multiplexer_upgrade = yamux::Config::default;
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg));

        SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(tcp_config, security_upgrade, multiplexer_upgrade)
            .unwrap()
            .with_behaviour(|key| {
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed((key.clone())),
                    gossipsub_config,
                );
                let mdns =
                    mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id());
                Ok(MyBehaviour { gossipsub, mdns })
            })
            .unwrap()
            .build()
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.swarm.next().await {
            info!("Swarm event: {:?}", event);
        }
    }

    /// Start listening on a given address
    pub fn listen(&mut self, address: &str) -> Result<(), Box<dyn Error>> {
        let addr: Multiaddr = address.parse()?;
        self.swarm.listen_on(addr)?;
        info!("Listening on {}", address);
        Ok(())
    }

    /// Dial a given address
    pub fn dial(&mut self, address: &str) -> Result<(), Box<dyn Error>> {
        let addr: Multiaddr = address.parse()?;
        self.swarm.dial(addr)?;
        info!("Dialing {}", address);
        Ok(())
    }
}

#[test]
fn instance_swarm() {
    let network = Libp2pNetwork::new();
    assert!(network.is_ok(), "Faild to create network");
}

#[tokio::test]
async fn instance() {
    let network = Libp2pNetwork::new();
    assert!(network.is_ok(), "Faild to create network");

    let mut network = network.unwrap();
    network.run().await;
}
#[tokio::test]
async fn test_listen_and_dial() {
    let mut network1 = Libp2pNetwork::new().expect("Failed to create network");
    let mut network2 = Libp2pNetwork::new().expect("Failed to create network");

    let listen_address = "/ip4/127.0.0.1/tcp/0"; // Use port 0 for automatic allocation

    assert!(
        network1.listen(listen_address).is_ok(),
        "Failed to start listening"
    );
    assert!(network2.dial(listen_address).is_ok(), "Failed to dial");
}
