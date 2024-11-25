pub mod auth;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/login", post(auth::login))
}

async fn hello() -> &'static str {
    "Hello from the API!"
}
