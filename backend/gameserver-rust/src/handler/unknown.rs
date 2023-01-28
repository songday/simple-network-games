use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use futures::{
    Future,
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use lazy_static_include::syn::token::Impl;
use tokio::sync::mpsc;

use crate::data::app::AppData;
use crate::data::room::RoomParams;
use crate::util::result::Error;

pub(crate) struct UnknownRoomWebSocketHandler;

impl CommonWebSocketHandler for UnknownRoomWebSocketHandler {
    fn new() -> Self {
        Self
    }

    async fn process_income_message(self, message: String) {
        println!("m={}", &m);
    }
}

pub(crate) trait CommonWebSocketHandler {
    fn new() -> Self;

    async fn websocket_handler<Fut>(
        self,
        room_params: Query<RoomParams>,
        ws: WebSocketUpgrade,
        State(app_data): State<Arc<AppData>>,
    ) -> impl IntoResponse where Self:Sized+Send {
        ws.on_upgrade(|websocket| async {
        let (tx, rx) = mpsc::channel::<String>(20);
        let mut room_idx = None;
        if room_params.room_id.is_none() || room_params.room_id.as_ref().unwrap().is_empty() {
            // create a new room
            let idx = app_data.new_room(&mut room_params, tx).await;
            match idx {
                Ok(idx) => room_idx = Some(idx),
                Err(e) => match e {
                    Error::Message(m) => {
                        super::send_msg_and_close(websocket, &m).await;
                        // return;
                    }
                },
            }
        } else {
            // Join a room
            let room_id = room_params.room_id.as_ref().unwrap();
            let mut idx_counter = 0usize;
            for r in app_data.rooms.lock().await.iter_mut() {
                if r.room_id.eq(room_id) {
                    r.add_player(room_params.player.clone(), tx);
                    room_idx = Some(idx_counter);
                    break;
                }
                idx_counter = idx_counter + 1;
            }
        }
        if room_idx.is_some() {
            let (sender, receiver) = websocket.split();
            let mut recv_task = tokio::spawn(async {
                let room_id = room_params.room_id.as_ref().unwrap();
                while let Some(Ok(message)) = receiver.next().await {
                    if let Message::Text(m) = message {
                        println!("m={}", &m);
                        self.process_income_message(m).await;
                    }
                }
                app_data.remove_room(&room_params).await;
                println!("read disconnected");
            });
            let mut send_task = tokio::spawn(async {
                while let Some(msg) = rx.recv().await {
                    println!("msg = {}", &msg);
                    if let Err(e) = sender.send(Message::Text(msg)).await {
                        rx.close();
                        eprintln!("err={:?}", e);
                        // client disconnected
                        break;
                    }
                }
                println!("write disconnected");
            });
            // tokio::select! {
            //     _ = (&mut send_task) => {
            //         println!("recv_task.abort");
            //         recv_task.abort();
            //     },
            //     _ = (&mut recv_task) => {
            //         println!("send_task.abort");
            //         send_task.abort();
            //     }
            // }
        } else {
            super::send_msg_and_close(websocket, "Cannot find the room").await;
        }
    });
}

    async fn process_income_message(self, message: String) where Self:Sized;
    
}

