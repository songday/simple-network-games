pub(crate) mod assets;
pub(crate) mod default;
pub(crate) mod draw;
pub(crate) mod lobby;
pub(crate) mod snake;
// pub(crate) mod unknown;

use axum::extract::ws::{Message, WebSocket};

pub(crate) async fn send_msg_and_close(mut websocket: WebSocket, msg: &str) {
    if let Err(e) = websocket.send(Message::Text(format!("E{}", msg))).await {
        eprintln!("{:?}", e);
    }
    if let Err(e) = websocket.close().await {
        eprintln!("{:?}", e);
    }
}
