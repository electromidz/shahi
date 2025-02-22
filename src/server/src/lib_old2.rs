use h3_quinn::quinn::{Endpoint, VarInt};
use quinn::crypto::rustls::QuicServerConfig;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use quinn::{rustls, ServerConfig, TransportConfig};
use rustls::crypto::ring;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tracing::{error, info};

pub struct Server;

impl Server {
    pub async fn start_server() -> Result<(), Box<dyn Error>> {
        // Install the default CryptoProvider (correct for rustls 0.23.23)
        ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");

        // Open and read the private key file
        let mut cert_chain_reader = BufReader::new(File::open("./public.pem")?);

        // Configure Rustls server
        let server_crypto = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)
            .expect("Failed to configure server");
        // // let key_der = PrivateKeyDer::Pkcs8(
        // //     private_key(&mut key_reader)
        // //         .expect("Failed to parse private key")
        // //         .expect("No private key found"),
        // // );
        //
        // // Open and read the certificate file
        // let cert_file = File::open("public.pem")?;
        // let mut cert_reader = BufReader::new(cert_file);
        //
        // let cert_chain: Vec<CertificateDer> = certs(&mut cert_reader)
        //     .expect("Failed to parse certificate")
        //     .into_iter()
        //     .map(CertificateDer::from)
        //     .collect();
        //
        // // Configure Rustls server
        // let server_crypto = ServerConfig::builder()
        //     .with_no_client_auth()
        //     .with_single_cert(cert_chain, key_der)
        //     .expect("Failed to configure server");
        //
        // println!("{:?}", server_crypto);
        //
        // // Create QUIC server configuration
        // let quic_config = QuicServerConfig::try_from(Arc::new(server_crypto))
        //     .expect("Failed to create QUIC config");
        // let server_config = ServerConfig::with_crypto(Arc::new(quic_config));

        println!("Here");
        ////---------------------------------------------------------
        //        // Install the default CryptoProvider
        //        rustls::crypto::ring::default_provider()
        //            .install_default()
        //            .expect("Failed to install rustls crypto provider");
        //        // Load server's private key and certificate
        //        // Open and read the private key file
        //        let key_file = File::open("private.pem")?;
        //        let mut key_reader = BufReader::new(key_file);
        //        //let key: Vec<u8> = fs::read("private.pem").expect("Failed to load key.pem file!");
        //        let cert: Vec<u8> = fs::read("public.pem").expect("Failed to load cert.pem file!");
        //        // Parse the private key and certificate
        //        let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_reader));
        //        let cert_chain = vec![CertificateDer::from(cert)];
        //
        //        // Configure Rustls server
        //        let server_crypto = rustls::ServerConfig::builder()
        //            .with_no_client_auth()
        //            .with_single_cert(cert_chain, key_der)
        //            .unwrap();
        //
        //        println!("{:?}", server_crypto);
        //        // Create QUIC server configuration
        //        let mut server_config =
        //            ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));
        //        println!("Here");
        //
        //        // Configure transport settings (optional)
        //        let mut transport_config = TransportConfig::default();
        //        transport_config.max_idle_timeout(Some(VarInt::from_u32(30_000).into())); // 30 seconds idle timeout
        //        server_config.transport_config(Arc::new(transport_config));
        //
        //        // Bind the server to the address
        //        let addr = "0.0.0.0:5050".parse().unwrap();
        //        let mut incoming = Endpoint::server(server_config, addr)?;
        //
        //        info!("Server listening on {}", addr);
        //
        //        // Accept incoming connections
        //        while let Some(connecting) = incoming.accept().await {
        //            let connection = connecting.await?;
        //            info!("New connection from: {}", connection.remote_address());
        //
        //            // Spawn a new task to handle the connection
        //            tokio::spawn(async move {
        //                if let Err(e) = handle_connection(connection).await {
        //                    error!("Connection error: {:?}", e);
        //                }
        //            });
        //        }
        Ok(())
    }
}

async fn handle_connection(connection: quinn::Connection) -> Result<(), Box<dyn Error>> {
    // Accept a new bidirectional stream
    let (mut send_stream, mut recv_stream) = connection.accept_bi().await?;

    // Read the request from the client
    let mut request = Vec::new();
    recv_stream.read_to_end(1024).await.unwrap();
    info!("Received request: {}", String::from_utf8_lossy(&request));

    // Prepare a response
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, QUIC!";

    // Send the response
    send_stream.write_all(response).await?;
    send_stream.finish();

    info!("Response sent to client");

    Ok(())
}
