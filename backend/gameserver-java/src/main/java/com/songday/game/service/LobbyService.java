package com.songday.game.service;

import com.songday.game.vo.RoomData;
import com.songday.game.vo.RoomType;
import io.github.scru128.Scru128;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;
import org.springframework.web.reactive.socket.WebSocketSession;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.locks.StampedLock;

@Service
public class LobbyService {
    private final List<RoomData> rooms = new ArrayList<>(16);
    private final StampedLock locker = new StampedLock();
    public static final String ATTR_ROOM_ID = "ROOM_ID";

    public RoomData newRoom(String creatorId, RoomType roomType, String roomName) {
        RoomData roomData = new RoomData();
        roomData.setRoomId(Scru128.generateString());
        roomData.setRoomName(roomName);
        roomData.setRoomType(roomType);
        roomData.setCreatorId(creatorId);
        final long lock = locker.readLock();
        try {
            rooms.add(roomData);
        } finally {
            locker.unlockRead(lock);
        }
        return roomData;
    }

    public List<RoomData> getRooms() {
        return rooms;
    }

    public RoomData getRoom(String roomId) {
        final long lock = locker.writeLock();
        try {
            for (RoomData room : rooms) {
                if (room.getRoomId().equals(roomId)) {
                    return room;
                }
            }
        } finally {
            locker.unlockWrite(lock);
        }
        return null;
    }

    public void removeRoom(String roomId) {
        final long lock = locker.writeLock();
        try {
            for (int i = 0; i < rooms.size(); i++) {
                if (rooms.get(i).getRoomId().equals(roomId)) {
                    rooms.remove(i);
                    break;
                }
            }
        } finally {
            locker.unlockWrite(lock);
        }
    }

    public void removeRoom(WebSocketSession session) {
        Object o = session.getAttributes().get(ATTR_ROOM_ID);
        if (o instanceof String) {
            removeRoom((String) o);
        }
    }

    public String[] getPlayers(WebSocketSession session) {
        Object o = session.getAttributes().get(LobbyService.ATTR_ROOM_ID);
        if (o instanceof String) {
            RoomData roomData = getRoom((String) o);
            if (roomData != null) {
                return roomData.getPlayers();
            }
        }
        return null;
    }

}
