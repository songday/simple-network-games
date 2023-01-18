use std::sync::Arc;
use std::vec::Vec;

use std::future::Future;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use super::room::{RoomData, RoomType};

pub(crate) struct AppData {
    /// Keys are the name of the channel
    pub(crate) rooms: Mutex<Vec<RoomData>>,
}

impl AppData {
    pub(crate) async fn new_room(
        &self,
        room_name: String,
        room_type: RoomType,
        player: String,
        channel_sender: Arc<mpsc::Sender<String>>,
    ) {
        let mut rooms = self.rooms.lock().await;
        let room = RoomData::new(room_name, room_type, player, channel_sender).await;
        rooms.push(room);
    }

    async fn get_room_and_do<F, T>(&self, room_id: &String, callback: F) -> Option<T>
    where
        F: FnOnce(&mut RoomData) -> T,
    {
        let mut rooms = self.rooms.lock().await;
        for r in rooms.iter_mut() {
            if r.room_id.eq(room_id) {
                return Some(callback(r));
            }
        }
        None
    }

    async fn get_room_and_do_async<F, Fut>(&self, room_id: &String, mut callback: F)
    where
        F: FnOnce(&mut RoomData) -> Fut,
        Fut: Future<Output = bool>,
    {
        let mut rooms = self.rooms.lock().await;
        for r in rooms.iter_mut() {
            if r.room_id.eq(room_id) {
                callback(r).await;
                return;
            }
        }
    }

    pub(crate) async fn join_room(
        &self,
        player: String,
        room_id: String,
        channel_sender: Arc<mpsc::Sender<String>>,
    ) -> bool {
        let f = |r: &mut RoomData| {
            r.add_player(player, channel_sender);
            true
        };
        // self.get_room_and_do(&room_id, |r| {r.add_player(player, channel_sender);true}).await;
        self.get_room_and_do(&room_id, f).await;
        false
    }

    pub(crate) async fn broadcast(&self, room_id: String, message: String) -> bool {
        // let mut rooms = self.rooms.lock().await;
        // for r in rooms.iter_mut() {
        //     if r.room_id.eq(&room_id) {
        //         r.broadcast_message(message).await;
        //         break;
        //     }
        // }
        let f = |r: &mut RoomData| r.broadcast_message(message);
        self.get_room_and_do_async(&room_id, f).await;
        false
    }
}
