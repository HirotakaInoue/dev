pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();
    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
