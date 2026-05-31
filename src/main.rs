use axum::{Router, http::StatusCode, routing::any};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let router = Router::new().route("/ws", any(ws_handler));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await
}

async fn ws_handler() -> StatusCode {
    StatusCode::OK
}
