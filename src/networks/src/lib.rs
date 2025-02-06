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
}

// pub async fn init_p_2_p() -> Result<(), Box<dyn Error>> {
//     let mut network = Libp2pNetwork::new().unwrap();
//     network.listen("/ip4/0.0.0.0/tcp/8080").unwrap();
//     network.run().await;
//
//     network.dial("/ip4/0.0.0.0/tcp/8080").unwrap();
//     Ok(())
// }
