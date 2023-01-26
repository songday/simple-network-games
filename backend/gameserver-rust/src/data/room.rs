use std::vec::Vec;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::util::result::{Error, Result};

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
    pub(crate) room_id: Option<String>,
    #[serde(alias = "rn", alias = "roomName")]
    pub(crate) room_name: Option<String>,
    pub(crate) player: String,
    #[serde(alias = "cap", alias = "capacity")]
    pub(crate) capacity: Option<u8>,
    #[serde(alias = "rty", alias = "roomType")]
    pub(crate) room_type: Option<RoomType>,
    #[serde(alias = "red", alias = "extraData")]
    pub(crate) extra_data: Option<String>,
}

impl RoomParams {
    pub(crate) fn as_room_data(&self) -> Result<RoomData> {
        if self.room_type.is_none() {
            return Err(Error::Message(String::from("Unknonw room type")));
        }
        if self.player.is_empty() {
            return Err(Error::Message(String::from("Player name is empty")));
        }
        let room_name = match &self.room_name {
            Some(n) => n,
            None => "",
        };
        if room_name.is_empty() || room_name.len() > 30 {
            return Err(Error::Message(String::from("Invalid room name")));
        }
        let room_type = self.room_type.as_ref().unwrap();
        let capacity = match room_type {
            RoomType::DRAW => 2u8,
            RoomType::SNAKE => 2u8,
            RoomType::UNKNOWN => match self.capacity {
                Some(c) => c,
                None => 0,
            },
        };
        Ok(RoomData {
            room_id: scru128::new_string(),
            room_name: String::from(room_name),
            room_type: room_type.clone(),
            players: Vec::with_capacity(capacity as usize),
            capacity,
            extra_data: match &self.extra_data {
                Some(d) => String::from(d),
                None => String::new(),
            },
            // Create a new channel for every room
            // tx: broadcast::channel(20).0,
        })
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
    ) -> Result<Self> {
        let mut room: RoomData = room_params.as_room_data()?;
        room.add_player(room_params.player.clone(), channel_sender);
        room.send_self_message(&room_params.player, format!("N{}", &room.room_id))
            .await;
        Ok(room)
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
