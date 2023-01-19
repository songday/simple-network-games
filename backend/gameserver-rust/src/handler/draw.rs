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
use crate::data::room::{RoomData, RoomParams};

pub(crate) async fn websocket_handler(
    room_params: Query<RoomParams>,
    ws: WebSocketUpgrade,
    State(app_data): State<Arc<AppData>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(room_params.0, socket, app_data))
}

async fn handle_websocket(
    mut room_params: RoomParams,
    mut websocket: WebSocket,
    app_data: Arc<AppData>,
) {
    let (tx, rx) = mpsc::channel::<String>(20);
    let mut room_idx = None;
    if room_params.room_id.is_empty() {
        // create a new room
        // let room = RoomData::new(&room_params, tx).await;
        // room_params.room_id = room.room_id.clone();
        let idx = app_data.new_room(&mut room_params, tx).await;
        room_idx = Some(idx);
    } else {
        let mut idx_counter = 0usize;
        for r in app_data.rooms.lock().await.iter() {
            if r.room_id.eq(&room_params.room_id) {
                room_idx = Some(idx_counter);
                break;
            }
            idx_counter = idx_counter + 1;
        }
    }
    if room_idx.is_some() {
        let (sender, receiver) = websocket.split();
        tokio::spawn(read(room_params, app_data, room_idx.unwrap(), receiver));
        tokio::spawn(write(sender, rx));
    } else {
        if let Err(e) = websocket.send(Message::Text(String::from(""))).await {
            eprintln!("{:?}", e);
        }
        if let Err(e) = websocket.close().await {
            eprintln!("{:?}", e);
        }
    }
}

async fn read(
    room_params: RoomParams,
    app_data: Arc<AppData>,
    room_idx: usize,
    // room_params: RoomParams,
    mut ws_receiver: SplitStream<WebSocket>,
    // app_data: Arc<AppData>,
    // channel_sender: mpsc::Sender<String>,
    // players: Arc<Mutex<Vec<(String, mpsc::Sender<String>)>>>,
) {
    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(m) = message {
            println!("m={}", &m);
            let cmd = &m[..1];
            if cmd.eq("P") {
                let rooms = app_data.rooms.lock().await;
                let room = rooms.get(room_idx);
                if room.is_some() {
                    let room = room.unwrap();
                    if room.room_id.eq(&room_params.room_id) {
                        room.send_message_to_others(&room_params.player, String::from(&m[1..]))
                            .await;
                    }
                }
            } else if cmd.eq("B") {
                let rooms = app_data.rooms.lock().await;
                let room = rooms.get(room_idx);
                if room.is_some() {
                    let room = room.unwrap();
                    if room.room_id.eq(&room_params.room_id) {
                        room.broadcast_message(String::from(&m[1..])).await;
                    }
                }
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
