use quinn::crypto::rustls::QuicServerConfig;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use std::sync::Arc;
mod auth;
mod route;
mod tls;
use clap::Parser;
use quinn::{rustls, ServerConfig, TransportConfig};

use h3_quinn::quinn::{Endpoint, VarInt};
use route::get_routes;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tls::load_tls_config;
use tracing::{error, info, info_span, instrument};

use std::fs;

pub struct Server;

impl Server {
    pub async fn start_server() -> Result<(), Box<dyn Error>> {
        // Install the default CryptoProvider
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");
        // First of all need to create cert and key

        let key: Vec<u8> = fs::read("key.pem").expect("Failed to load key.pem file!");
        let cert: Vec<u8> = fs::read("cert.pem").expect("Failed to load cert.pem file!");

        // should be handle if we dont have kem.pem return error
        let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key));

        let cert_chain = vec![CertificateDer::from(cert)];

        let server_crypto = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)?;

        let mut server_config =
            quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());

        let addr: SocketAddr = "0.0.0.0:443".parse()?; // Ensure address is set
        let endpoint = quinn::Endpoint::server(server_config, addr)?;
        eprintln!("listening on {}", endpoint.local_addr()?);

        let connection_limit = Some(10);
        let block_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let block = Some(block_socket);
        let stateless_retry = true;
        let root = get_routes();
        while let Some(conn) = endpoint.accept().await {
            let remote_addr = conn.remote_address();
            if connection_limit.is_some_and(|n| endpoint.open_connections() >= n) {
                info!("Refusing connection from {}: limit reached", remote_addr);
                conn.refuse();
                continue;
            }

            if Some(remote_addr) == block {
                info!("Refusing blocked client IP address: {}", remote_addr);
                conn.refuse();
                continue;
            }

            if stateless_retry && !conn.remote_address_validated() {
                info!(
                    "Requiring connection from {} to validate address",
                    remote_addr
                );
                conn.retry().unwrap();
                continue;
            }

            info!("Accepting connection from {}", remote_addr);
            let fut = handle_connection(root.clone(), conn);
            tokio::spawn(async move {
                if let Err(e) = fut.await {
                    error!("Connection failed: {:?}", e);
                }
            });
        }

        Ok(())
    }
}

async fn handle_connection(root: Arc<Path>, conn: quinn::Incoming) -> Result<()> {
    let connection = conn.await?;
    let span = info_span!(
        "connection",
        remote = %connection.remote_address(),
        protocol = %connection
            .handshake_data()
            .unwrap()
            .downcast::<quinn::crypto::rustls::HandshakeData>().unwrap()
            .protocol
            .map_or_else(|| "<none>".into(), |x| String::from_utf8_lossy(&x).into_owned())
    );
    async {
        info!("established");

        // Each stream initiated by the client constitutes a new request.
        loop {
            let stream = connection.accept_bi().await;
            let stream = match stream {
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    info!("connection closed");
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(s) => s,
            };
            let fut = handle_request(root.clone(), stream);
            tokio::spawn(
                async move {
                    if let Err(e) = fut.await {
                        error!("failed: {reason}", reason = e.to_string());
                    }
                }
                .instrument(info_span!("request")),
            );
        }
    }
    .instrument(span)
    .await?;
    Ok(())
}

// let addr: SocketAddr = "0.0.0.0:443".parse()?; // Ensure address is set
//                                                // load TLS config
// let tls_config = load_tls_config().unwrap();
//
// // Create QUIC endpoint
// let routes = get_routes();
// let mut server_config =
//     ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls_config)?));
//
// // Optimize QUIC transport
// let mut transport_config = TransportConfig::default();
// transport_config.max_concurrent_uni_streams(VarInt::from_u32(100)); // Fixed method usage
// transport_config.max_idle_timeout(Some(VarInt::from_u32(10_000).into())); // Optional: Add timeout
//
// server_config.transport = Arc::new(transport_config);
//
// //let endpoint = quinn::Endpoint::server(server_config, addr);
// let endpoint = Endpoint::server(server_config, addr).unwrap();
//
// // Handle incoming connections
// while let Some(new_conn) = endpoint.accept().await {
//     // Here you would typically handle the connection, for example:
//     // spawn a task to handle the connection or use it directly.
//     tokio::spawn(async move {
//         // Handle new connection here
//         println!("New connection: {:?}", new_conn);
//     });
// }
