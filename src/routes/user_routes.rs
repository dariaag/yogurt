use axum::{routing::get, routing::post, Router};

use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers;

pub fn user_routes() -> Router {
    Router::new()
        .route(
            "/users",
            get(handlers::get_all_users_handler).post(handlers::add_user_handler),
        )
        //.route("/delete_all", post(handlers::delete_all_users_handler))
        .route("/studios", get(handlers::get_all_studios_handler))
}
