use std::net::SocketAddr;

use router::create_router;

pub mod app_state;
use app_state::AppState;

mod database;
mod router;
mod routes;
pub mod utilities;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
