use std::sync::Arc;
use std::vec::Vec;

use serde::Deserialize;
use tokio::sync::mpsc;

pub(crate) enum RoomType {
    DRAW,
    SNAKE,
}

#[derive(Deserialize)]
pub(crate) struct RoomParams {
    pub(crate) room_id: String,
    pub(crate) room_name: String,
    pub(crate) player: String,
}

pub(crate) struct RoomData {
    /// Previously stored in AppRoom
    pub(crate) room_id: String,
    pub(crate) room_name: String,
    pub(crate) room_type: RoomType,
    players: Vec<(String, Arc<mpsc::Sender<String>>)>,
    pub(crate) capacity: u8,
    pub(crate) extra_data: String,
    // tx: broadcast::Sender<String>,
}

impl RoomData {
    pub(crate) async fn new(
        room_name: String,
        room_type: RoomType,
        player: String,
        channel_sender: Arc<mpsc::Sender<String>>,
    ) -> Self {
        let capacity = match room_type {
            RoomType::DRAW => 2u8,
            RoomType::SNAKE => 2u8,
        };
        let mut room = Self {
            room_id: scru128::new_string(),
            room_name,
            room_type,
            players: Vec::with_capacity(2),
            capacity,
            extra_data: String::with_capacity(128),
            // Create a new channel for every room
            // tx: broadcast::channel(20).0,
        };
        let send_to = player.clone();
        room.add_player(player, channel_sender);
        room.send_self_message(&send_to, String::from(&room.room_id))
            .await;
        room
    }

    pub(crate) fn add_player(&mut self, player: String, channel_sender: Arc<mpsc::Sender<String>>) {
        self.players.push((player, channel_sender));
    }

    pub(crate) async fn send_self_message(&self, player: &String, message: String) -> bool {
        for p in self.players.iter() {
            if player.eq(&p.0) {
                if let Err(e) = p.1.send(message).await {
                    eprintln!("err={:?}", e);
                    return false;
                }
                return true;
            }
        }
        return false;
    }

    pub(crate) async fn send_message_to_others(&self, player: &String, message: String) -> bool {
        for p in self.players.iter() {
            if !player.eq(&p.0) {
                if let Err(e) = p.1.send(message.clone()).await {
                    eprintln!("err={:?}", e);
                }
                return true;
            }
        }
        return false;
    }

    pub(crate) async fn broadcast_message(&self, message: String) -> bool {
        for p in self.players.iter() {
            if let Err(e) = p.1.send(message.clone()).await {
                eprintln!("err={:?}", e);
            }
        }
        return false;
    }
}
