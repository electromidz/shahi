use quinn::crypto::rustls::QuicServerConfig;
use std::{net::SocketAddr, sync::Arc};
mod auth;
mod route;
mod tls;

use h3_quinn::quinn::Endpoint;
use route::get_routes;
use std::error::Error;
use tls::load_tls_config;

pub struct Server;

impl Server {
    pub async fn start_server() -> Result<(), Box<dyn Error>> {
        let addr: SocketAddr = "0.0.0.0:443".parse().unwrap();
        // load TLS config
        let tls_config = load_tls_config();
        // Create QUIC endpoint
        let routes = get_routes();

        let server_config = quinn::ServerConfig::with_crypto(Arc::new(
            QuicServerConfig::try_from(tls_config).unwrap(),
        ));

        //let endpoint = quinn::Endpoint::server(server_config, addr);
        let endpoint = Endpoint::server(server_config, addr)?;

        // Handle incoming connections
        while let Some(new_conn) = endpoint.accept().await {
            // Here you would typically handle the connection, for example:
            // spawn a task to handle the connection or use it directly.
            tokio::spawn(async move {
                // Handle new connection here
                println!("New connection: {:?}", new_conn);
            });
        }
        Ok(())
    }
}
