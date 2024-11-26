pub mod auth;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn create_routes(db_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/login", post(auth::login))
        .layer(Extension(db_pool))
}

async fn hello() -> &'static str {
    "Hello from the API!"
}
