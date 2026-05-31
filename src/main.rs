use axum::{Router, routing::any};
use rtc_server::websocket::{echo::ws_echo_handler, rtc::ws_handler};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let router = Router::new()
        .route("/ws", any(ws_handler))
        .route("/ws/echo", any(ws_echo_handler));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await
}
