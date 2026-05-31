use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};

pub async fn ws_echo_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ws_echo_callback)
}

async fn ws_echo_callback(mut socket: WebSocket) {
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
