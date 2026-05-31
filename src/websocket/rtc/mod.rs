use axum::{
    extract::{WebSocketUpgrade, ws::WebSocket},
    response::Response,
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ws_callback)
}

async fn ws_callback(mut _socket: WebSocket) {}
