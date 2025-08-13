mod handlers;
mod routes;
mod models;
mod utils;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use routes::youtube_routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
                .route("/", get(|| async { "Hello, Rust Backend!!!" }))
                .merge(youtube_routes());

    let port = std::env::var("PORT").unwrap_or("8080".into());
    let addr = format!("0.0.0.0:{}", port);

    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
