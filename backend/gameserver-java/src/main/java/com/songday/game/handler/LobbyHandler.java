package com.songday.game.handler;

import com.songday.game.service.LobbyService;
import com.songday.game.util.JsonUtils;
import lombok.AllArgsConstructor;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.reactive.socket.WebSocketHandler;
import org.springframework.web.reactive.socket.WebSocketSession;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.io.IOException;
import java.util.concurrent.atomic.AtomicInteger;

@AllArgsConstructor
@Slf4j
public class LobbyHandler implements WebSocketHandler {
    private final LobbyService lobbyService;
    private final AtomicInteger onlinePlayerAmount = new AtomicInteger(0);

    @Override
    public Mono<Void> handle(WebSocketSession session) {
        onlinePlayerAmount.incrementAndGet();
        String message;
        try {
            message = JsonUtils.getObjectMapper().writeValueAsString(lobbyService.getRooms());
        } catch (IOException e) {
            log.error(e.getMessage(), e);
            message = "[{}]";
        }
        return session.send(Flux.just(session.textMessage(message))).doOnTerminate(onlinePlayerAmount::decrementAndGet);
    }
}
