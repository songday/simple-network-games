package com.songday.game.vo;

import lombok.Data;

@Data
public class RoomData {
    private String roomId;
    private String roomName;
    private RoomType roomType;
    private String creatorId;
}
