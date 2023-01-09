package com.songday.game.handler;

import com.songday.game.service.LobbyService;
import com.songday.game.util.JsonUtils;
import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.reactive.socket.WebSocketHandler;
import org.springframework.web.reactive.socket.WebSocketSession;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.io.IOException;
import java.time.Duration;
import java.util.concurrent.atomic.AtomicInteger;

@AllArgsConstructor
@Slf4j
public class LobbyHandler implements WebSocketHandler {
    private final LobbyService lobbyService;
    private final AtomicInteger onlinePlayerAmount = new AtomicInteger(0);

    private String getMessage() {
        try {
            return JsonUtils.getObjectMapper().writeValueAsString(lobbyService.getRooms());
        } catch (IOException e) {
            log.error(e.getMessage(), e);
            return "[{}]";
        }
    }

    @Override
    public Mono<Void> handle(WebSocketSession session) {
        onlinePlayerAmount.incrementAndGet();
        final String message = getMessage();
        return session.send(Flux.interval(Duration.ofMillis(0L), Duration.ofMillis(5000L)).map(l -> session.textMessage(message))).doOnTerminate(() -> {
            log.info("Finished");
            onlinePlayerAmount.decrementAndGet();
        });
    }
}
