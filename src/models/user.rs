use std::sync::Arc;

use crate::utils::serialize_dt::serialize_dt;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::{FromRow, PgPool};
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
    role: String,
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    role: String,
    #[serde(serialize_with = "serialize_dt")]
    created_at: DateTime<Utc>,
}
impl User {
    pub async fn create(new_user: NewUser, pool: &PgPool) -> Result<User, sqlx::Error> {
        let hashed_password = new_user.password;
        let mut tx = pool.begin().await?;
        let user = sqlx::query_as::<_, User>("INSERT INTO users (username, email, password, role) VALUES ($1, $2, $3, $4) RETURNING id, username, email, password, role, created_at")
            .bind(&new_user.username)
            .bind(&new_user.email)
            .bind(&hashed_password) // Ensure hashed password is used
            .bind(&new_user.role)
            .fetch_one(&mut *tx) // Change the type of tx to &mut PgConnection
            .await?;
        tx.commit().await?;
        Ok(user)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_all(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users").execute(pool).await?;
        Ok(())
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password.as_bytes()
    }
}
