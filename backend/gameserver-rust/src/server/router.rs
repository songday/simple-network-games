use std::sync::Arc;
use std::vec::Vec;

use axum::{
    response::Html,
    routing::get,
    Router,
};
use tokio::sync::Mutex;

use crate::data::app::AppData;
use crate::handler::{draw, lobby, snake};

pub(crate) fn get_route() -> Router {
    let app_data = Arc::new(AppData {
        rooms: Mutex::new(Vec::with_capacity(16)),
    });

    Router::new()
        .route("/", get(lobby))
        .route("/lobby", get(lobby::websocket_handler))
        .route("/room/draw", get(draw::websocket_handler))
        .route("/room/snake", get(snake::websocket_handler))
        .with_state(app_data)
}

async fn lobby() -> Html<&'static str> {
    Html(std::include_str!("../../../../frontend/html/lobby.html"))
}
