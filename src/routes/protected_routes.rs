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

pub fn protected_routes(shared_pool: std::sync::Arc<sqlx::Pool<sqlx::Postgres>>) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = DbBackend { pool: shared_pool };
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    Router::new()
        //.route("/login", post(login))
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(DbBackend, login_url = "/login"))
        .route("/login", post(user_handlers::login))
        .layer(auth_layer)
}
