use crate::auth::login;
use axum::{routing::post, Router};

pub fn get_routes() -> Router {
    Router::new().route("/login", post(login))
}
