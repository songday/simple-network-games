use std::vec::Vec;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum RoomType {
    DRAW,
    SNAKE,
    UNKNOWN,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoomParams {
    #[serde(alias = "ri", alias = "roomId")]
    pub(crate) room_id: String,
    #[serde(alias = "rn", alias = "roomName")]
    pub(crate) room_name: String,
    pub(crate) player: String,
    #[serde(alias = "cap", alias = "capacity")]
    pub(crate) capacity: u8,
    #[serde(alias = "rty", alias = "roomType")]
    pub(crate) room_type: RoomType,
    #[serde(alias = "red", alias = "extraData")]
    pub(crate) extra_data: String,
}

impl std::convert::From<&RoomParams> for RoomData {
    fn from(r: &RoomParams) -> Self {
        let capacity = match r.room_type {
            RoomType::DRAW => 2u8,
            RoomType::SNAKE => 2u8,
            RoomType::UNKNOWN => r.capacity,
        };
        Self {
            room_id: scru128::new_string(),
            room_name: r.room_name.clone(),
            room_type: r.room_type.clone(),
            players: Vec::with_capacity(capacity as usize),
            capacity,
            extra_data: r.extra_data.clone(),
            // Create a new channel for every room
            // tx: broadcast::channel(20).0,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoomData {
    /// Previously stored in AppRoom
    pub(crate) room_id: String,
    pub(crate) room_name: String,
    pub(crate) room_type: RoomType,
    // players: Vec<(String, mpsc::Sender<String>)>,
    #[serde(skip_serializing)]
    players: Vec<(String, mpsc::Sender<String>)>,
    pub(crate) capacity: u8,
    pub(crate) extra_data: String,
    // tx: broadcast::Sender<String>,
}

impl RoomData {
    pub(crate) async fn new(
        room_params: &RoomParams,
        channel_sender: mpsc::Sender<String>,
    ) -> Self {
        let mut room: RoomData = room_params.into();
        room.add_player(room_params.player.clone(), channel_sender);
        room.send_self_message(&room_params.player, String::from(&room.room_id))
            .await;
        room
    }

    pub(crate) fn add_player(&mut self, player: String, channel_sender: mpsc::Sender<String>) {
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

    pub(crate) fn clear_players(&mut self) {
        self.players.clear();
    }
}
