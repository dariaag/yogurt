use crate::auth::auth::DbBackend;
use crate::handlers::{admin_handlers, public_handlers, user_handlers};
use axum::{routing::get, routing::post, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

pub fn user_routes() -> Router {
    Router::new()
        //.route("/users", get(admin_handlers::get_all_users_handler))
        .route("/logout", get(user_handlers::logout))
        .route("/profile", get(user_handlers::get_profile))
        .route_layer(login_required!(DbBackend, login_url = "/login"))
}
