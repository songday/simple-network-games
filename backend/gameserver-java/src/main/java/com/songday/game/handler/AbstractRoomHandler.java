package com.songday.game.handler;

import com.songday.game.service.LobbyService;
import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.util.StringUtils;
import org.springframework.web.reactive.socket.WebSocketHandler;
import org.springframework.web.reactive.socket.WebSocketSession;
import reactor.core.publisher.Mono;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

@Slf4j
@AllArgsConstructor
public abstract class AbstractRoomHandler implements WebSocketHandler {
    protected final LobbyService lobbyService;
    protected final Map<String, List<String>> allMessages = new ConcurrentHashMap<>(10, 1f);

    protected void addBroadcastMessages(WebSocketSession session, String message) {
        String[] players = lobbyService.getPlayers(session);
        addMessage(players[0], message);
        addMessage(players[1], message);
    }

    protected void addMessage(String sessionId, String message) {
        allMessages.computeIfAbsent(sessionId, k -> new ArrayList<>(16)).add(message);
    }

    protected void addSelfMessage(WebSocketSession session, String message) {
        addMessage(session.getId(), message);
    }

    protected List<String> getSelfMessages(WebSocketSession session) {
        List<String> cachedMessages = allMessages.computeIfAbsent(session.getId(), k -> new ArrayList<>(16));
        List<String> returnMessages = new ArrayList<>(cachedMessages);
        cachedMessages.clear();
        return returnMessages;
    }

    protected void addCompetitorMessage(WebSocketSession session, String message) {
        String[] players = lobbyService.getPlayers(session);
        String competitorId = players[1];
        if (StringUtils.hasText(competitorId)) {
            addMessage(competitorId, message);
        }
    }

    protected void cleanUp(WebSocketSession session) {
        allMessages.remove(session.getId());
        lobbyService.removeRoom(session);
    }

    @Override
    public abstract Mono<Void> handle(WebSocketSession session);
}
