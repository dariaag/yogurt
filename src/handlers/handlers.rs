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
pub async fn add_studio_handler(
    pool: Extension<Arc<PgPool>>,
    extract::Json(payload): extract::Json<Studio>,
) -> impl IntoResponse {
    match Studio::create(payload, &pool).await {
        Ok(_) => (StatusCode::CREATED, "Studio created successfully"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Studio creation failed"),
    }
}
#[debug_handler]
pub async fn get_all_studios_handler(pool: Extension<Arc<PgPool>>) -> impl IntoResponse {
    let studios = Studio::find_all(&pool).await;
    let empty_vec: Vec<Studio> = vec![];
    match studios {
        Ok(studios) => (StatusCode::OK, Json(studios)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(empty_vec)),
    }
}

type AuthSession = axum_login::AuthSession<DbBackend>;
#[debug_handler]
pub async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => {
            println!("auth");
            user
        }
        Ok(None) => {
            println!("Not authorized");
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/protected").into_response()
}
