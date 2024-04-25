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
