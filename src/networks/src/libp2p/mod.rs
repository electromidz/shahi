pub mod types; // Declare the `types` module

use std::error::Error;
use std::time::Duration;

use libp2p::swarm::{NetworkBehaviour, Swarm, SwarmEvent};
use libp2p::{noise, ping, tcp, yamux, Multiaddr, SwarmBuilder};
use types::InitResult; // Import `InitResult` from the `types` module

struct MyBehaviour {
    ping: ping::Behaviour,
}

pub struct Network {
    swarm: Swarm<MyBehaviour>,
}

impl Network {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let behaviour = MyBehaviour {
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
        };

        let swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(60)) // 60 seconds timeout
            })
            .with_behaviour(|_| behaviour)?
            .build();

        Ok(Network { swarm })
    }

    pub fn listen(&mut self, addr: Multiaddr) -> InitResult {
        self.swarm.listen_on(addr)?;
        Ok(true)
    }

    pub fn dial(&mut self, addr: Multiaddr) -> InitResult {
        self.swarm.dial(addr)?;
        Ok(true)
    }

    pub fn run(&mut self) {
        tokio::spawn(async move {
            loop {
                match self.swarm.select_next_some().await {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {}", address);
                    }
                    SwarmEvent::Behaviour(event) => {
                        println!("Behaviour event: {:?}", event);
                    }
                    _ => {}
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ping_server() -> Result<(), Box<dyn std::error::Error>> {
        let init_server = Network::init()?; // Unwrap the Result
        assert_eq!(init_server, true); // Compare the inner bool value
        Ok(()) // Return `Ok(())` for tests
    }
}
