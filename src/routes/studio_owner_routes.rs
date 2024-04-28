use crate::auth::auth::DbBackend;
use crate::handlers::{admin_handlers, public_handlers, studio_handlers, user_handlers};
use axum::handler::Handler; // Add this import
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_login::permission_required;
pub fn studio_owner_routes() -> Router {
    Router::new()
        //.route("/delete_all", post(handlers::delete_all_users_handler))
        .route("/studios", post(studio_handlers::add_studio_handler))
        .route("/classes", post(studio_handlers::add_class_handler))
        .route_layer(permission_required!(DbBackend, "studio_owner"))
}
