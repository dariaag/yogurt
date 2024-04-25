use crate::handlers;
use axum::handler::Handler; // Add this import
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

pub fn public_routes() -> Router {
    Router::new()
        .route(
            "/users",
            get(handlers::get_all_users_handler).post(handlers::add_user_handler),
        )
        //.route("/delete_all", post(handlers::delete_all_users_handler))
        .route("/studios", get(handlers::get_all_studios_handler))
}
