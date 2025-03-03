use bytes::Buf;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::ServerConfig;
mod auth;
mod route;
mod utils;
mod handler;

use h3_quinn::quinn::Endpoint;
use quinn::crypto::rustls::QuicServerConfig;
use quinn::rustls;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::task;

use bytes::Bytes;
use h3::server::Connection as H3Connection;
use h3::server::RequestStream;
use h3_quinn::BidiStream;
use h3_quinn::Connection as H3QuinnConnection;
use tracing::{error, info, warn};

use http::{Response, StatusCode};
use utils::{load_certificate_chain, load_private_key};
use handler::{handle_http3_connection, handle_http3_request};

pub struct Server;

impl Server {
    pub async fn start_server() -> Result<(), Box<dyn Error>> {
        // Install the default CryptoProvider
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");

        let key_der = load_private_key("certs/server.key")?;
        // Load the certificate chain
        let cert_chain = load_certificate_chain("certs/server.crt")?;

        // Build the server configuration
        let mut server_crypto = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)
            .expect("Failed to create server config");

        // Enable ALPN for HTTP/3
        server_crypto.alpn_protocols = vec![b"h3".to_vec()];

        //FIXME: if you want ot see he config on server just enbale info
        // Print the server configuration for debugging
        // info!(
        //     "Server configuration created successfully: \n{:?}\n",
        //     server_crypto
        // );

        // Start your server logic here
        // For example, you can use `server_config` with Quinn or another library.
        let server_config =
            quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

        let server = Endpoint::server(server_config, "0.0.0.0:8080".parse().unwrap()).unwrap();
        info!("Server is running on {:?}", server.local_addr());

        // Accept incoming connections
        while let Some(connecting) = server.accept().await {
            // Spawn a new async task for each connection
            task::spawn(async move {
                match connecting.await {
                    Ok(connection) => {
                        info!("New connection: {:?}", connection.remote_address());
                        //println!("{:?}  - {:?}\n", connection.stats(), connection.rtt());
                        handle_http3_connection(connection).await;
                    }
                    Err(e) => {
                        error!("Failed to establish connection: {:?}", e);
                    }
                }
            });
        }
        Ok(())
    }
}


