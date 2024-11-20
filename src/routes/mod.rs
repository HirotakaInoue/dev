use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello))
}

async fn hello() -> &'static str {
    "Hello from the API!"
}
