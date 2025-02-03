use libp2p::{
    core::transport::Transport,
    identity::Keypair,
    noise, ping,
    swarm::{NetworkBehaviour, Swarm, SwarmBuilder},
    tcp, yamux,
};
use std::{error::Error, time::Duration};

mod types;

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub ping: ping::Behaviour,
}

#[warn(dead_code)]
pub struct Libp2pNetwork {
    swarm: Swarm<MyBehaviour>, // ✅ Fixed typo
}

impl Libp2pNetwork {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let keypair = Keypair::generate_ed25519();
        let transport = tcp::Config::default()
            .and_then(noise::Config::new)
            .and_then(yamux::Config::default)
            .boxed();

        let behaviour = MyBehaviour {
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
        };

        let swarm = SwarmBuilder::new(transport, behaviour, keypair).build();

        Ok(Self { swarm })
    }
}

#[test]
#[test]
fn instance_swarm() {
    use super::*;

    let network = Libp2pNetwork::new().expect("Failed to create Libp2pNetwork");

    // Ensure swarm is initialized
    assert!(network.swarm.is_connected());
}
