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
    auth::{
        self,
        auth::{Credentials, DbBackend, Permission},
    },
    models::{
        class::{Class, NewClass},
        studio::{NewStudio, Studio},
        user::{self, NewUser, User},
    },
};
use axum_login::AuthzBackend;

use axum_login::AuthSession;

#[debug_handler]
pub async fn add_studio_handler(
    pool: Extension<Arc<PgPool>>,
    auth_session: Extension<axum_login::AuthSession<DbBackend>>,
    extract::Json(payload): extract::Json<NewStudio>,
) -> impl IntoResponse {
    let user = auth_session.user.as_ref().unwrap();

    let has_perm = auth_session
        .backend
        .has_perm(&user, Permission::from("studio_owner"))
        .await;
    println!("{:?}", has_perm);
    match Studio::create(payload, &pool).await {
        Ok(_) => (StatusCode::CREATED, "Studio created successfully"),
        Err(e) => {
            println!("{:?}", e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, "Studio creation failed")
        }
    }
}

#[debug_handler]
pub async fn add_class_handler(
    pool: Extension<Arc<PgPool>>,
    extract::Json(payload): extract::Json<NewClass>,
) -> impl IntoResponse {
    match Class::create(payload, &pool).await {
        Ok(_) => (StatusCode::CREATED, "Class created successfully"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Class creation failed"),
    }
}
