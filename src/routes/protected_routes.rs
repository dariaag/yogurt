use crate::auth::auth::DbBackend;
use crate::handlers::user_handlers;
use axum::{
    routing::{get, post},
    Router,
};
use axum_login::{
    login_required,
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};

pub fn protected_routes() -> Router {
    Router::new()
        //.route("/login", post(login))
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route("/logged_in", get(|| async { "Logged in!" }))
        .route_layer(login_required!(DbBackend, login_url = "/login"))
}
