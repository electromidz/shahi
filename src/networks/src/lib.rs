mod libp2p;

use libp2p::DummyBehaviour;

use ::libp2p::Swarm;
use libp2p::Libp2pNetwork;

pub struct Network;

impl Network {
    pub async fn init() -> Libp2pNetwork {
        Libp2pNetwork::new().unwrap()
    }
    pub async fn create() -> Swarm<DummyBehaviour> {
        Libp2pNetwork::create_swarm().await
    }

    pub async fn create_gossip() -> Swarm {}
}
