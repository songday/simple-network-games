#[macro_export]
macro_rules! common_websocket_handler {
    ($expression:expr) => {
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
        use crate::data::room::RoomParams;
        use crate::util::result::Error;

        pub(crate) async fn websocket_handler(
            room_params: Query<RoomParams>,
            ws: WebSocketUpgrade,
            State(app_data): State<Arc<AppData>>,
        ) -> impl IntoResponse {
            ws.on_upgrade(|socket| handle_websocket(room_params.0, socket, app_data))
        }

        async fn handle_websocket(
            mut room_params: RoomParams,
            websocket: WebSocket,
            app_data: Arc<AppData>,
        ) {
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
                            return;
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
                let mut recv_task =
                    tokio::spawn(read(room_params, app_data, room_idx.unwrap(), receiver));
                let mut send_task = tokio::spawn(write(sender, rx));
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
        }

        async fn read(
            room_params: RoomParams,
            app_data: Arc<AppData>,
            room_idx: usize,
            mut ws_receiver: SplitStream<WebSocket>,
        ) {
            let room_id = room_params.room_id.as_ref().unwrap();
            while let Some(Ok(message)) = ws_receiver.next().await {
                if let Message::Text(m) = message {
                    println!("m={}", &m);
                    let cmd = &m[..1];
                    $expression();
                }
            }
            app_data.remove_room(&room_params).await;
            println!("read disconnected");
        }

        async fn write(
            mut ws_sender: SplitSink<WebSocket, Message>,
            mut channel_receiver: mpsc::Receiver<String>,
        ) {
            while let Some(msg) = channel_receiver.recv().await {
                println!("msg = {}", &msg);
                if let Err(e) = ws_sender.send(Message::Text(msg)).await {
                    channel_receiver.close();
                    eprintln!("err={:?}", e);
                    // client disconnected
                    break;
                }
            }
            println!("write disconnected");
        }
    };
}

macro_rules! test_m {
    ($ident:ident) => {
        impl core::fmt::Write for $ident {
            fn write_str(&mut self, _s: &str) -> core::fmt::Result {
                unimplemented!();
           }
        }

        impl $ident {
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
            use crate::data::room::RoomParams;
            use crate::util::result::Error;

            pub(crate) async fn websocket_handler(
                room_params: Query<RoomParams>,
                ws: WebSocketUpgrade,
                State(app_data): State<Arc<AppData>>,
            ) -> impl IntoResponse {
                ws.on_upgrade(|socket| handle_websocket(room_params.0, socket, app_data))
            }

            async fn handle_websocket(
                mut room_params: RoomParams,
                websocket: WebSocket,
                app_data: Arc<AppData>,
            ) {
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
                                return;
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
                    let mut recv_task =
                        tokio::spawn(read(room_params, app_data, room_idx.unwrap(), receiver));
                    let mut send_task = tokio::spawn(write(sender, rx));
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
            }

            async fn read(
                room_params: RoomParams,
                app_data: Arc<AppData>,
                room_idx: usize,
                mut ws_receiver: SplitStream<WebSocket>,
            ) {
                let room_id = room_params.room_id.as_ref().unwrap();
                while let Some(Ok(message)) = ws_receiver.next().await {
                    if let Message::Text(m) = message {
                        println!("m={}", &m);
                        let cmd = &m[..1];
                        $expression();
                    }
                }
                app_data.remove_room(&room_params).await;
                println!("read disconnected");
            }

            async fn write(
                mut ws_sender: SplitSink<WebSocket, Message>,
                mut channel_receiver: mpsc::Receiver<String>,
            ) {
                while let Some(msg) = channel_receiver.recv().await {
                    println!("msg = {}", &msg);
                    if let Err(e) = ws_sender.send(Message::Text(msg)).await {
                        channel_receiver.close();
                        eprintln!("err={:?}", e);
                        // client disconnected
                        break;
                    }
                }
                println!("write disconnected");
            }
        }
    };
}
