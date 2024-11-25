pub mod db;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    // DBの接続
    let db_pool = db::connection::create_pool().await.unwrap();

    let app = routes::create_routes(db_pool);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
