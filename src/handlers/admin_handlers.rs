use axum::{
    async_trait,
    extract::{self, Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use std::{sync::Arc, vec};

use axum::{debug_handler, routing::get, Router};
use axum::{response::Redirect, Form};

use crate::{
    auth::auth::{Credentials, DbBackend},
    models::{
        studio::Studio,
        user::{NewUser, User},
    },
};

#[debug_handler]
pub async fn delete_all_users_handler(pool: Extension<Arc<PgPool>>) -> impl IntoResponse {
    match User::delete_all(&pool).await {
        Ok(_) => (StatusCode::OK, "All users deleted successfully"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete all users",
        ),
    }
}

#[debug_handler]
pub async fn get_user_by_id_handler(
    extract::Path(id): extract::Path<i32>,
    pool: Extension<Arc<PgPool>>,
) -> impl IntoResponse {
    match User::find_by_id(id, &pool).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default())),
    }
}

#[debug_handler]
pub async fn get_all_users_handler(pool: Extension<Arc<PgPool>>) -> impl IntoResponse {
    let users = User::find_all(&pool).await;
    let empty_vec: Vec<User> = vec![];
    match users {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(empty_vec)),
    }
}
