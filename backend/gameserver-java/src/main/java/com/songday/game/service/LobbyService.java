package com.songday.game.service;

import com.songday.game.vo.RoomData;
import com.songday.game.vo.RoomType;
import io.github.scru128.Scru128;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;

@Service
public class LobbyService {
    private final List<RoomData> rooms = new ArrayList<>(16);
    private final Object locker = new Object();

    public RoomData newRoom(String creatorId, RoomType roomType, String roomName) {
        RoomData roomData = new RoomData();
        roomData.setRoomId(Scru128.generateString());
        roomData.setRoomName(roomName);
        roomData.setRoomType(roomType);
        roomData.setCreatorId(creatorId);
        synchronized (locker) {
            rooms.add(roomData);
        }
        return roomData;
    }

    public List<RoomData> getRooms() {
        return rooms;
    }

    public RoomData getRoom(String roomId) {
        synchronized (locker) {
            for (RoomData room : rooms) {
                if (room.getRoomId().equals(roomId)) {
                    return room;
                }
            }
        }
        return null;
    }

    public void removeRoom(String roomId) {
        synchronized (locker) {
            for (int i = 0; i < rooms.size(); i++) {
                if (rooms.get(i).getRoomId().equals(roomId)) {
                    rooms.remove(i);
                    break;
                }
            }
        }
    }
}
