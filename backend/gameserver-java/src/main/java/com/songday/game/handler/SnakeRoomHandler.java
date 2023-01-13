package com.songday.game.handler;

import com.songday.game.service.LobbyService;
import com.songday.game.vo.RoomData;
import com.songday.game.vo.RoomType;
import com.songday.game.vo.SnakeRoomData;
import lombok.extern.slf4j.Slf4j;
import org.springframework.util.StringUtils;
import org.springframework.web.reactive.socket.WebSocketMessage;
import org.springframework.web.reactive.socket.WebSocketSession;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.Duration;

@Slf4j
public class SnakeRoomHandler extends AbstractRoomHandler {

    public SnakeRoomHandler(LobbyService lobbyService) {
        super(lobbyService);
    }

    private Mono<Void> send(WebSocketSession session) {
        Flux<String> sendMessages = Flux.interval(Duration.ofMillis(1500L)).flatMap(l -> Flux.fromIterable(getSelfMessages(session))).doOnComplete(() -> log.info("send complete"));
        return session.send(sendMessages.map(session::textMessage));
    }

    @Override
    public Mono<Void> handle(WebSocketSession session) {
        log.info("new connection id={}", session.getId());
        Mono<Void> input = session.receive().doOnNext(payload -> {
            if (!WebSocketMessage.Type.TEXT.equals(payload.getType()))
                return;
            final String m = payload.getPayloadAsText();
            log.info("m={}", m);
            if (m.isEmpty())
                return;
            // New room
            final char cmd = m.charAt(0);
            if (cmd == 'N') {
                String data = m.substring(1);
                String[] dataArray = StringUtils.tokenizeToStringArray(data, "|", true, true);
                if (dataArray.length != 3)
                    throw new RuntimeException("Wrong room data");
                RoomData roomData = lobbyService.newRoom(session.getId(), RoomType.SNAKE, dataArray[0]);
                if (roomData == null) {
                    throw new RuntimeException("Create game room failed");
                } else {
                    String[] players = new String[]{session.getId(), ""};
                    roomData.setPlayers(players);

                    SnakeRoomData snakeRoomData = new SnakeRoomData();
                    snakeRoomData.setRefreshIntervalMillis(Integer.parseInt(dataArray[1]));
                    snakeRoomData.setGameMode(dataArray[2]);
                    roomData.setSnakeRoomData(snakeRoomData);
                    session.getAttributes().put(LobbyService.ATTR_ROOM_ID, roomData.getRoomId());
                    addSelfMessage(session, "N" + roomData.getRoomId());
                }
            }
            // Join a room
            else if (cmd == 'J') {
                String joinRoomId = m.substring(1);
                RoomData roomData = lobbyService.getRoom(joinRoomId);
                if (roomData == null) {
                    throw new RuntimeException("Can not found game room by roomId " + joinRoomId);
                } else {
                    String[] players = roomData.getPlayers();
                    players[1] = session.getId();
                    session.getAttributes().put(LobbyService.ATTR_ROOM_ID, roomData.getRoomId());

                    SnakeRoomData snakeRoomData = roomData.getSnakeRoomData();
                    addBroadcastMessages(session, "J" + snakeRoomData.getRefreshIntervalMillis() + "|" + snakeRoomData.getGameMode());
                }
            }
            // Pass through
            else if (cmd == 'P') {
                addCompetitorMessage(session, m.substring(1));
            }
            // Boardcast
            else if (cmd == 'B') {
                addBroadcastMessages(session, m.substring(1));
            }
        }).doOnComplete(
            () -> {
                log.info("Connection disconnected");
                cleanUp(session);
            }
        ).doOnError(e -> {
            log.error(e.getMessage(), e);
            cleanUp(session);
        }).then();

        Mono<Void> output = send(session);

        return Mono.zip(input, output).then();

    }
}
