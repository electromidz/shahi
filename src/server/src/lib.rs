use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::ServerConfig;
mod auth;
mod route;
mod tls;
use h3_quinn::quinn::Endpoint;
use quinn::crypto::rustls::QuicServerConfig;
use quinn::rustls;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

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
        let server_crypto = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)
            .expect("Failed to create server config");

        // Print the server configuration for debugging
        println!(
            "Server configuration created successfully: {:?}\n",
            server_crypto
        );

        // Start your server logic here
        // For example, you can use `server_config` with Quinn or another library.
        let server_config =
            quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

        let server = Endpoint::server(server_config, "127.0.0.1:5050".parse().unwrap()).unwrap();
        println!("Server is running on 127.0.0.1:5050");

        // Accept incoming connections
        while let Some(connecting) = server.accept().await {
            let connection = connecting.await?;
            println!("New connection: {:?}", connection.remote_address());
        }

        // while let Some(connecting) = server.accept().await {
        //     let connection = connecting.await?;
        //     println!("{:?}  - {:?}\n", connection.stats(), connection.rtt());
        // }

        Ok(())
    }
}

fn load_private_key(path: &str) -> Result<PrivateKeyDer, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    // Parse the PEM file and extract PKCS#8 private keys
    let keys: Vec<PrivatePkcs8KeyDer> =
        pkcs8_private_keys(&mut reader).collect::<Result<Vec<_>, _>>()?;

    // Check if any keys were found
    if keys.is_empty() {
        return Err("No PKCS#8 private keys found in the file".into());
    }

    // Use the first key (you can modify this logic if multiple keys are expected)
    let key = keys.into_iter().next().unwrap(); // Safe to unwrap because we checked `is_empty`
    Ok(PrivateKeyDer::Pkcs8(key))
}

fn load_certificate_chain(path: &str) -> Result<Vec<CertificateDer>, Box<dyn Error>> {
    //regenerate key with this
    //openssl req -x509 -newkey rsa:2048 -keyout server.key -out server.crt -days 365 -nodes -subj "/CN=localhost"
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Parse the PEM file and extract certificates
    let certs: Vec<CertificateDer> = certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(CertificateDer::from)
        .collect();

    // Check if any certificates were found
    if certs.is_empty() {
        return Err("No certificates found in the file".into());
    }

    Ok(certs)
}
