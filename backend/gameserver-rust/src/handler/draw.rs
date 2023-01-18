use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use tokio::sync::mpsc;

use crate::data::app::AppData;
use crate::data::room::{RoomParams, RoomType};

pub(crate) async fn websocket_handler(
    room_params: Query<RoomParams>,
    ws: WebSocketUpgrade,
    State(app_data): State<Arc<AppData>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(room_params, socket, app_data))
}

async fn handle_websocket(
    room_params: Query<RoomParams>,
    websocket: WebSocket,
    app_data: Arc<AppData>,
) {
    let (sender, receiver) = websocket.split();
    let (tx, rx) = mpsc::channel::<String>(20);
    tokio::spawn(read(room_params, receiver, app_data.clone(), tx));
    tokio::spawn(write(sender, rx));
}

async fn read(
    room_params: Query<RoomParams>,
    mut ws_receiver: SplitStream<WebSocket>,
    app_data: Arc<AppData>,
    channel_sender: mpsc::Sender<String>,
) {
    let channel_sender = Arc::new(channel_sender);
    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(m) = message {
            println!("m={}", &m);
            let cmd = &m[..1];
            if cmd.eq("N") {
                app_data
                    .new_room(
                        String::from(&m[1..]),
                        RoomType::DRAW,
                        room_params.player.clone(),
                        channel_sender.clone(),
                    )
                    .await;
            } else if cmd.eq("J") {
                app_data
                    .join_room(
                        room_params.player.clone(),
                        String::from(&m[1..]),
                        channel_sender.clone(),
                    )
                    .await;
            } else if cmd.eq("P") {
                app_data
                    .join_room(
                        room_params.player.clone(),
                        String::from(&m[1..]),
                        channel_sender.clone(),
                    )
                    .await;
            } else if cmd.eq("B") {
                app_data
                    .join_room(
                        room_params.player.clone(),
                        String::from(&m[1..]),
                        channel_sender.clone(),
                    )
                    .await;
            }
        }
    }
}

async fn write(
    mut ws_sender: SplitSink<WebSocket, Message>,
    mut channel_receiver: mpsc::Receiver<String>,
) {
    while let Some(msg) = channel_receiver.recv().await {
        println!("msg = {}", &msg);
        if let Err(e) = ws_sender.send(Message::Text(msg)).await {
            // client disconnected
            eprintln!("err={:?}", e);
            return;
        }
    }
}
