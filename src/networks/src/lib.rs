pub mod libp2p;

use libp2p::{ MyBehaviour };
pub use libp2p::DummyBehaviour;

use ::libp2p::{Multiaddr, Swarm, };
use libp2p::Libp2pNetwork;
use std::error::Error;
pub use ::libp2p::swarm::DialError;
use libp2p::gossipsub;

pub struct Network;

impl Network {
    pub async fn init() -> Libp2pNetwork {
        Libp2pNetwork::new().unwrap()
}
    pub async fn create() -> Swarm<DummyBehaviour> {
        Libp2pNetwork::create_swarm().await
    }
    pub async fn create_gossip() -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
        match MyBehaviour::create_gossip_swarm().await {
            Ok(mut swarm) => {
                let topic = gossipsub::IdentTopic::new("test-net");
                swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

                // Listen on all interfaces with dynamically assigned ports
                swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
                swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
                Ok(swarm)
            }
            Err(e) => Err(e.into()),
        }
    }
    pub async fn dial(network: &mut Swarm<DummyBehaviour>, address: Multiaddr) -> Result<(),DialError> {
        match network.dial(address) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub async fn listen(network: &mut Swarm<DummyBehaviour>, address: Multiaddr) -> Result<(),Box<dyn Error>> {
        match network.listen_on(address) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
    pub async fn start_gossip()-> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
        MyBehaviour::start_gossip().await
    }
}
