use axum::{routing::get, routing::post, Router};

use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::{admin_handlers, public_handlers, user_handlers};

pub fn user_routes() -> Router {
    Router::new().route(
        "/users",
        get(admin_handlers::get_all_users_handler).post(user_handlers::add_user_handler),
    )
    //.route("/delete_all", post(handlers::delete_all_users_handler))
}
