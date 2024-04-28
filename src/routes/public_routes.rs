use crate::handlers::{admin_handlers, public_handlers, user_handlers};
use axum::handler::Handler; // Add this import
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

pub fn public_routes() -> Router {
    Router::new()
        .route("/users", post(public_handlers::add_user_handler))
        //.route("/delete_all", post(handlers::delete_all_users_handler))
        .route("/studios", get(public_handlers::get_all_studios_handler))
        .route("/classes", get(public_handlers::get_all_classes_handler))
        .route(
            "/check_session",
            get(public_handlers::check_session_handler),
        )
        .route("/login", post(public_handlers::login))
}
