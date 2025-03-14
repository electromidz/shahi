mod libp2p;

use libp2p::{DummyBehaviour, MyBehaviour};

use ::libp2p::Swarm;
use libp2p::Libp2pNetwork;
use std::error::Error;

pub struct Network;

impl Network {
    pub async fn init() -> Libp2pNetwork {
        Libp2pNetwork::new().unwrap()
    }
    pub async fn create() -> Swarm<DummyBehaviour> {
        Libp2pNetwork::create_swarm().await
    }
    pub async fn create_gossip() -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
        MyBehaviour::crete_gossip_swap().await
    }
}
