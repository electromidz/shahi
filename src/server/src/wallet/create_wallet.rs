use std::error::Request;

pub fn create_wallet(request: Request) {
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
                error!("Create wallet");
                return send_json_response(
                    &mut stream,
                    StatusCode::BAD_REQUEST,
                )
                    .await;
            }
        };
}