use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

const SECRET_KEY: &[u8] = b"super_secret_key_123"; // Replace with a real secret

pub async fn login(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    if payload.username == "bob" && payload.password == "password123" {
        let claims = serde_json::json!({ "sub": payload.username });
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET_KEY),
        )
        .unwrap();
        Json(LoginResponse { token })
    } else {
        Json(LoginResponse {
            token: "Invalid credentials".to_string(),
        })
    }
}
