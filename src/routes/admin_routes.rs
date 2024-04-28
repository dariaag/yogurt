use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::admin_handlers;

pub fn admin_routes() -> Router {
    Router::new()
        .route(
            "/delete_all",
            post(admin_handlers::delete_all_users_handler),
        )
        .route("/users", get(admin_handlers::get_all_users_handler))
        .route("/user/:id", get(admin_handlers::get_user_by_id_handler))
    //.route_layer(permission_required!(DbBackend, "admin"))
}
