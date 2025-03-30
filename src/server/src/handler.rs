use bytes::{Buf, Bytes};
use db::port::BlockchainDB;
use db::RocksDBAdapter;
use account::{Account, State};
use h3::server::Connection as H3Connection;
use h3::server::RequestStream;
use h3_quinn::BidiStream;
use h3_quinn::Connection as H3QuinnConnection;
use http::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Serialize, Deserialize, Debug)]
struct UserRegistration {
    address: String,
    public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserAddress {
    address: String,
}

pub async fn handle_http3_connection(connection: quinn::Connection) {
    // Wrap the QUIC connection in an HTTP/3 connection
    let mut h3_conn = match H3Connection::new(H3QuinnConnection::new(connection)).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to establish HTTP/3 connection: {:?}", e);
            return;
        }
    };

    let rocksdb_adapter: Arc<dyn BlockchainDB> =
        Arc::new(RocksDBAdapter::new("/home/o/Music/db").expect("Failed to create DB!"));

    // Accept incoming HTTP/3 requests
    while let Ok(Some((request, stream))) = h3_conn.accept().await {
        let db_clone = Arc::clone(&rocksdb_adapter); // Clone inside the loop

        // Spawn a new task to handle the request
        tokio::spawn(async move {
            if let Err(e) = handle_http3_request(request, stream, db_clone).await {
                error!("Failed to handle HTTP/3 request: {:?}", e);
            }
        });
    }
}

pub async fn handle_http3_request(
    request: http::Request<()>,
    mut stream: RequestStream<BidiStream<Bytes>, Bytes>, // Use h3_quinn::BidiStream
    db: Arc<dyn BlockchainDB>,
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

            let user: UserRegistration = match serde_json::from_str(&body_str) {
                Ok(user) => user,
                Err(_) => {
                    error!("Registered");
                    return send_json_response(
                        &mut stream,
                        StatusCode::BAD_REQUEST,
                    )
                    .await;
                }
            };

            let mut account = State::new();
            State::create_account(&mut account, user.address.clone(), user.public_key.clone());
            let acc = State::get_account(&mut account, &user.address);
            match acc {
                Some(account) => {
                    info!("Account created: {:?}", account);
                    db.create_account(&account);
                },
                None => warn!("Account does not exist: {:?}", user.address),
            };





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
        ("/getAddress", &http::Method::POST) => {
            let mut body = Vec::new();

            // Read the request body data
            while let Some(chunk) = stream.recv_data().await? {
                body.extend_from_slice(&chunk.chunk());
            }

            // Convert body to a string (assuming JSON)
            let body_str = String::from_utf8(body)?;

            let address: UserAddress = match serde_json::from_str(&body_str) {
                Ok(addr) => addr,
                Err(_) => {
                    error!("thuis error");
                    return send_json_response(
                        &mut stream,
                        StatusCode::BAD_REQUEST,
                    )
                    .await;
                }
            };

            db.get_account(address.address.clone());
            let balance = db.get_balance(address.address.clone()).unwrap();
            warn!("Your balance: {}", balance);

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

// Read request body
async fn receive_body(
    stream: &mut RequestStream<BidiStream<Bytes>, Bytes>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = Vec::new();
    while let Some(chunk) = stream.recv_data().await? {
        body.extend_from_slice(&chunk.chunk());
    }
    String::from_utf8(body).map_err(|e| e.into())
}

// Helper function to send JSON responses
async fn send_json_response(
    stream: &mut RequestStream<BidiStream<Bytes>, Bytes>,
    status: StatusCode,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(())?;

    stream.send_response(response).await?;
    stream
        .send_data(Bytes::from(r#"{"error": "Invalid JSON"}"#))
        .await?;

    Ok(())
}
