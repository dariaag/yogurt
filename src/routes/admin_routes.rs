use axum::{routing::post, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::admin_handlers as handlers;

pub fn admin_routes() -> Router {
    Router::new().route("/delete_all", post(handlers::delete_all_users_handler))
}
