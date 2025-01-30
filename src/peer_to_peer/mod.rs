use libp2p::{identity, PeerId};
use std::error::Error;

pub struct PeerToPeer {}

impl PeerToPeer {
    pub async fn init() -> Result<(), Box<dyn Error>> {
        println!("init p2p network");
        // Generate a keypair and peer ID
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        println!("Local peer ID: {:?}", peer_id);

        // Create a transport (TCP + Noise for encryption)
        // let transp = TokioTcpConfig::new()
        //     .upgrade(upgrade::Version::V1)
        //     .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        //     .multiplex(mplex::MplexConfig::new())
        //     .boxed();

        Ok(())
        // // Create a Ping behavior (for testing)
        // let behavior = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
        //
        // // Create a Swarm to manage the network
        // let mut swarm = Swarm::new(transport, behavior, peer_id);
        //
        // // Listen on a local address
        // swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        //
        // println!("Listening for connections...");
        //
        // // Main event loop
        // loop {
        //     match swarm.select_next_some().await {
        //         SwarmEvent::NewListenAddr { address, .. } => {
        //             println!("Listening on {:?}", address);
        //         }
        //         SwarmEvent::Behaviour(event) => {
        //             println!("Received event: {:?}", event);
        //         }
        //         _ => {}
        //     }
        // }
    }
}
