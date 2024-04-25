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
        studio::{NewStudio, Studio},
        user::{NewUser, User},
    },
};

#[debug_handler]
pub async fn add_studio_handler(
    pool: Extension<Arc<PgPool>>,
    extract::Json(payload): extract::Json<NewStudio>,
) -> impl IntoResponse {
    match Studio::create(payload, &pool).await {
        Ok(_) => (StatusCode::CREATED, "Studio created successfully"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Studio creation failed"),
    }
}
