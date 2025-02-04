mod libp2p;
use std::error::Error;

use libp2p::Libp2pNetwork;

pub async fn init_p_2_p() -> Result<(), Box<dyn Error>> {
    let mut network = Libp2pNetwork::new().unwrap();
    network.listen("/ip4/0.0.0.0/tcp/8080").unwrap();

    network.run().await;
    Ok(())
}
