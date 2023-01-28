use crate::common_websocket_handler;

common_websocket_handler!({
    if cmd.eq("S") {
        println!("received S");
        let rooms = app_data.rooms.lock().await;
        let room = rooms.get(room_idx);
        if room.is_some() {
            let room = room.unwrap();
            if room.room_id.eq(room_id) {
                room.send_message_to_others(&room_params.player, String::from(&m[1..]))
                    .await;
            }
        }
    } else if cmd.eq("B") {
        let rooms = app_data.rooms.lock().await;
        let room = rooms.get(room_idx);
        if room.is_some() {
            let room = room.unwrap();
            if room.room_id.eq(room_id) {
                room.broadcast_message(String::from(&m[1..])).await;
            }
        }
    }
});
