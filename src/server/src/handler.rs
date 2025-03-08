use bytes::{Buf, Bytes};
use db::RocksDBAdapter;
use h3::server::Connection as H3Connection;
use h3::server::RequestStream;
use h3_quinn::BidiStream;
use h3_quinn::Connection as H3QuinnConnection;
use http::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

#[derive(Serialize, Deserialize, Debug)]
struct UserRegistration {
    address: String,
    public_key: Vec<u8>,
}

pub async fn handle_http3_connection(connection: quinn::Connection) {
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

pub async fn handle_http3_request(
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

            let user: UserRegistration = match serde_json::from_str(&body_str) {
                Ok(user) => user,
                Err(_) => {
                    return send_json_response(
                        &mut stream,
                        StatusCode::BAD_REQUEST,
                        r#"{"error": "Invalid JSON"}"#,
                    )
                    .await;
                }
            };

            let rocksdb_adapter =
                RocksDBAdapter::new("/home/o/Music/db").expect("Failed to created DB!");

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

// Helper function to send JSON responses
async fn send_json_response(
    stream: &mut RequestStream<BidiStream<Bytes>, Bytes>,
    status: StatusCode,
    body: &str,
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
