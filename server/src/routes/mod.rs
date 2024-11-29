pub mod auth;
pub mod set_api_key;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn create_routes(db_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/login", post(auth::login))
        .route("/set_api_key", post(set_api_key::set_api_key))
        .with_state(db_pool)
}

async fn hello() -> &'static str {
    "Hello from the API!"
}
