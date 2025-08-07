mod handlers;
mod routes;
mod models;
mod utils;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use routes::youtube_routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
                .route("/", get(|| async { "Hello, Rust Backend!!!" }))
                .merge(youtube_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
