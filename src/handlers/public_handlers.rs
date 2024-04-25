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
pub async fn get_all_studios_handler(pool: Extension<Arc<PgPool>>) -> impl IntoResponse {
    let studios = Studio::find_all(&pool).await;
    let empty_vec: Vec<Studio> = vec![];
    match studios {
        Ok(studios) => (StatusCode::OK, Json(studios)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(empty_vec)),
    }
}
