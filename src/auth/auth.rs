use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::user::User;
#[derive(Clone)]
pub struct DbBackend {
    pub pool: Arc<PgPool>,
}

impl DbBackend {
    pub fn new(pool: Arc<PgPool>) -> Self {
        DbBackend { pool }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub user_id: i32,
}

type AuthSession = axum_login::AuthSession<DbBackend>;

#[async_trait]
impl AuthnBackend for DbBackend {
    type User = User;
    type Credentials = Credentials; // Define this based on your login form
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(&credentials.user_id)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(user)
    }
}
