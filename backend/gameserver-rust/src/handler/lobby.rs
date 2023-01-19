use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use tokio::time::{sleep, Duration};

use crate::data::app::AppData;

pub(crate) async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_data): State<Arc<AppData>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, app_data))
}

async fn handle_websocket(mut websocket: WebSocket, app_data: Arc<AppData>) {
    loop {
        let rooms = app_data.rooms.lock().await;
        let json = serde_json::to_string(&(*rooms)).unwrap();
        if let Err(e) = websocket.send(Message::Text(json)).await {
            eprintln!("lobby send err={:?}", e);
            break;
        }
        sleep(Duration::from_millis(2000)).await;
    }
}
