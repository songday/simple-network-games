use std::net::SocketAddr;

use super::router;

pub async fn start_app() {
    let r = router::get_route();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(r.into_make_service())
        .await
        .unwrap();
}
