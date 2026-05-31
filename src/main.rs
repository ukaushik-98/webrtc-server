use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{
            Message::{self},
            WebSocket,
        },
    },
    response::Response,
    routing::any,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let router = Router::new().route("/ws", any(ws_handler));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ws_callback)
}

async fn ws_callback(mut socket: WebSocket) {
    while let Some(maybe_msg) = socket.recv().await {
        if let Ok(msg) = maybe_msg {
            let msg_text = msg.to_text().unwrap();
            let response = format!("Server Echo: {}", msg_text);
            let resp_msg = Message::Text(response.into());
            let _ = socket.send(resp_msg).await;
        } else {
            // client disconnected
            return;
        }
    }
}
