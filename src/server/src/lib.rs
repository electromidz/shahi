use bytes::Buf;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::ServerConfig;
mod auth;
mod route;
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

async fn handle_http3_connection(connection: quinn::Connection) {
    // Wrap the QUIC connection in an HTTP/3 connection
    let mut h3_conn: H3Connection<H3QuinnConnection, Bytes> =
        H3Connection::new(H3QuinnConnection::new(connection))
            .await
            .unwrap();

    // Accept incoming HTTP/3 requests
    while let Ok(Some((request, mut stream))) = h3_conn.accept().await {
        // Spawn a new task to handle the request
        tokio::spawn(async move {
            error!("Failed to handle HTTP/3 request: {:?}", request);
            if let Err(e) = handle_http3_request(request, stream).await {
                error!("Failed to handle HTTP/3 request: {:?}", e);
            }
        });
    }
}

async fn handle_http3_request(
    request: http::Request<()>,
    mut stream: RequestStream<BidiStream<Bytes>, Bytes>, // Use h3_quinn::BidiStream
) -> Result<(), Box<dyn std::error::Error>> {
    // Match the request path and method
    match (request.uri().path(), request.method()) {
        ("/login", &http::Method::GET) => {
            warn!("REQUEST{:?}", request);
            // Create a JSON response for the /login route
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(())?;

            // Send the response headers
            stream.send_response(response).await?;

            // Send the response body
            stream
                .send_data(Bytes::from(r#"{"message": true}"#))
                .await?;
        }
        ("/register", &http::Method::POST) => {
            let mut body = Vec::new();

            // Read the request body data
            while let Some(chunk) = stream.recv_data().await? {
                body.extend_from_slice(&chunk.chunk());
            }

            // Convert body to a string (assuming JSON)
            let body_str = String::from_utf8(body)?;

            warn!("Received POST body: {}", body_str);
            // Create a JSON response for the /register route
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(())?;

            // Send the response headers
            stream.send_response(response).await?;

            // Send the response body
            stream
                .send_data(Bytes::from(r#"{"message": true}"#))
                .await?;
        }
        _ => {
            // Return a 404 Not Found response for unknown routes
            let response = Response::builder().status(StatusCode::NOT_FOUND).body(())?;

            // Send the response headers
            stream.send_response(response).await?;
        }
    }

    // Finish the stream
    stream.finish().await?;

    Ok(())
}
