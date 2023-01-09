package com.songday.game.handler;

import com.songday.game.service.LobbyService;
import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.reactive.HandlerMapping;
import org.springframework.web.reactive.config.EnableWebFlux;
import org.springframework.web.reactive.handler.SimpleUrlHandlerMapping;
import org.springframework.web.reactive.socket.WebSocketHandler;
import org.springframework.web.reactive.socket.server.WebSocketService;
import org.springframework.web.reactive.socket.server.support.HandshakeWebSocketService;
import org.springframework.web.reactive.socket.server.support.WebSocketHandlerAdapter;
import org.springframework.web.reactive.socket.server.upgrade.ReactorNettyRequestUpgradeStrategy;

import java.util.HashMap;
import java.util.Map;

@Configuration
@EnableWebFlux
@RequiredArgsConstructor
public class WebSocketConfig {
    private final LobbyService lobbyService;
    @Bean
    public HandlerMapping handlerMapping() {
        Map<String, WebSocketHandler> map = new HashMap<>(8, 1f);
        map.put("/lobby", new LobbyHandler(lobbyService));
        map.put("/room/draw", new DrawRoomHandler(lobbyService));
        map.put("/room/snake", new SnakeRoomHandler(lobbyService));
        int order = -1;

        return new SimpleUrlHandlerMapping(map, order);
    }

    @Bean
    public WebSocketHandlerAdapter handlerAdapter() {
        return new WebSocketHandlerAdapter(webSocketService());
    }

    @Bean
    public WebSocketService webSocketService() {
        ReactorNettyRequestUpgradeStrategy reactorNettyRequestUpgradeStrategy = new ReactorNettyRequestUpgradeStrategy();
        return new HandshakeWebSocketService(reactorNettyRequestUpgradeStrategy);
    }
}
