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
        studio::Studio,
        user::{NewUser, User},
    },
};
use axum::{debug_handler, routing::get, Router};
use axum::{response::Redirect, Form};
use axum_login::{AuthSession, AuthUser, AuthnBackend, UserId};

#[debug_handler]
pub async fn logout(mut auth_session: AuthSession<DbBackend>) -> impl IntoResponse {
    auth_session.logout().await.unwrap();
    StatusCode::OK.into_response()
}

#[debug_handler]

pub async fn get_profile(auth_session: AuthSession<DbBackend>) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => (StatusCode::OK, Json(user)),
        None => (StatusCode::UNAUTHORIZED, Json(User::default())),
    }
}
