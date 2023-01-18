use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use futures::{sink::SinkExt, stream::StreamExt};

use crate::data::app::AppData;

pub(crate) async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_data): State<Arc<AppData>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, app_data))
}

async fn handle_websocket(websocket: WebSocket, app_data: Arc<AppData>) {
    let (mut sender, mut receiver) = websocket.split();
}
