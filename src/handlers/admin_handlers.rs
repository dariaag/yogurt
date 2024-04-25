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
pub async fn add_user_handler(
    pool: Extension<Arc<PgPool>>,
    extract::Json(payload): extract::Json<NewUser>,
) -> impl IntoResponse {
    match User::create(payload, &pool).await {
        Ok(_) => (StatusCode::CREATED, "User created successfully"),
        Err(e) => {
            println!("{:?}", e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, "User creation failed")
        }
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
