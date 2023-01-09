package com.songday.game.handler;

import com.songday.game.service.LobbyService;
import com.songday.game.vo.RoomData;
import com.songday.game.vo.RoomType;
import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.reactive.socket.WebSocketHandler;
import org.springframework.web.reactive.socket.WebSocketMessage;
import org.springframework.web.reactive.socket.WebSocketSession;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

@Slf4j
@AllArgsConstructor
public class DrawRoomHandler implements WebSocketHandler {
    private final LobbyService lobbyService;
    private final Map<String, String> playerPairs = new ConcurrentHashMap<>(10, 1f);
    private final Map<String, List<String>> allMessages = new ConcurrentHashMap<>(10, 1f);
    private final String ATTR_ROOM_ID = "ROOM_ID";

    private void removeRoom(WebSocketSession session) {
        Object o = session.getAttributes().get(ATTR_ROOM_ID);
        if (o instanceof String) {
            lobbyService.removeRoom((String) o);
        }
    }

    private void addBroadcastMessages(WebSocketSession session, String message) {
        addSelfMessage(session, message);
        String roomCreator = playerPairs.get(session.getId());
        if (roomCreator != null) {
            addCompetitorMessage(session, message);
        } else {
            for (Map.Entry<String, String> entry : playerPairs.entrySet()) {
                if (session.getId().equals(entry.getValue())) {
                    addMessage(entry.getKey(), message);
                    break;
                }
            }
        }
    }

    private void addMessage(String sessionId, String message) {
        allMessages.computeIfAbsent(sessionId, k -> new ArrayList<>(16)).add(message);
    }

    private void addSelfMessage(WebSocketSession session, String message) {
        addMessage(session.getId(), message);
    }

    private List<String> getSelfMessages(WebSocketSession session) {
        List<String> cachedMessages = allMessages.computeIfAbsent(session.getId(), k -> new ArrayList<>(16));
        List<String> returnMessages = new ArrayList<>(cachedMessages);
        cachedMessages.clear();
        return returnMessages;
    }

    private List<String> getCompetitorMessages(WebSocketSession session) {
        String competitorId = playerPairs.get(session.getId());
        if (competitorId == null)
            return null;
        return allMessages.computeIfAbsent(competitorId, k -> new ArrayList<>(16));
    }

    private void addCompetitorMessage(WebSocketSession session, String message) {
        String competitorId = playerPairs.get(session.getId());
        if (competitorId != null) {
            List<String> messages = getCompetitorMessages(session);
            if (messages != null)
                messages.add(message);
        }
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
                RoomData roomData = lobbyService.newRoom(session.getId(), RoomType.DRAW, m.substring(1));
                if (roomData == null) {
                    throw new RuntimeException("Create game room failed");
                } else {
                    session.getAttributes().put(ATTR_ROOM_ID, roomData.getRoomId());
                    addSelfMessage(session, "N" + roomData.getRoomId());
                    playerPairs.put(roomData.getRoomId(), session.getId());
                }
            }
            // Join a room
            else if (cmd == 'J') {
                String joinRoomId = m.substring(1);
                RoomData roomData = lobbyService.getRoom(joinRoomId);
                if (roomData == null) {
                    throw new RuntimeException("Can not found game room by roomId " + joinRoomId);
                } else {
                    session.getAttributes().put(ATTR_ROOM_ID, roomData.getRoomId());
                    playerPairs.put(roomData.getCreatorId(), session.getId());
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
                removeRoom(session);
            }
        ).doOnError(e -> {
            log.error(e.getMessage(), e);
            removeRoom(session);
        }).then();

        Mono<Void> output = send(session);

        return Mono.zip(input, output).then();

    }
}
