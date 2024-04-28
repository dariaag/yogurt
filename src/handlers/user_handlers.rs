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
use axum_login::{AuthUser, AuthnBackend, UserId};

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

#[debug_handler]
pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    auth_session.logout().await.unwrap();
    StatusCode::OK.into_response()
}
