use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::routes::{hello_world, users::*};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hello_world::root))
        .route("/create-user", post(create_user::create_user))
        .with_state(app_state)
}
