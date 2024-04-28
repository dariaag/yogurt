use axum::{
    async_trait,
    extract::{self, Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use std::{sync::Arc, vec};

use crate::{
    auth::{
        self,
        auth::{Credentials, DbBackend},
    },
    models::{
        class::Class,
        studio::Studio,
        user::{NewUser, User},
    },
};
use axum::{debug_handler, routing::get, Router};
use axum::{response::Redirect, Form};
use axum_login::{tower_sessions::session_store, AuthSession};
use axum_login::{AuthUser, AuthnBackend, UserId};
#[debug_handler]
pub async fn get_all_studios_handler(pool: Extension<Arc<PgPool>>) -> impl IntoResponse {
    let studios = Studio::find_all(&pool).await;
    let empty_vec: Vec<Studio> = vec![];
    match studios {
        Ok(studios) => (StatusCode::OK, Json(studios)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(empty_vec)),
    }
}

#[debug_handler]
pub async fn get_all_classes_handler(pool: Extension<Arc<PgPool>>) -> impl IntoResponse {
    let classes = Class::find_all(&pool).await;
    let empty_vec: Vec<Class> = vec![];
    match classes {
        Ok(classes) => (StatusCode::OK, Json(classes)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(empty_vec)),
    }
}

#[debug_handler]
pub async fn check_session_handler(auth_session: AuthSession<DbBackend>) -> impl IntoResponse {
    match auth_session.user {
        Some(_) => (StatusCode::OK, "Session is active"),
        None => (StatusCode::UNAUTHORIZED, "Session is inactive"),
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
pub async fn login(
    mut auth_session: AuthSession<DbBackend>,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => {
            println!("{:?}", user.session_auth_hash());
            println!("auth");
            user
        }
        Ok(None) => {
            println!("Not authorized");
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // if auth_session.login(&user).await.is_err() {
    //     return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    // }
    // println!("Logged in");

    // println!("{:?}", auth_session.user);

    // Redirect::to("/logged_in").into_response()

    auth_session.login(&user).await.unwrap();
    StatusCode::OK.into_response()
}
