use axum::async_trait;
use axum_login::{AuthnBackend, AuthzBackend, UserId};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::{collections::HashSet, sync::Arc};

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

#[async_trait]
impl AuthnBackend for DbBackend {
    type User = User;
    type Credentials = Credentials; // Define this based on your login form
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        println!("CALLING AUTHENTICATE");
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(&credentials.user_id)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        println!("CALLING GET USER");
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(user)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, FromRow)]
pub struct Permission {
    pub name: String,
}

impl From<&str> for Permission {
    fn from(name: &str) -> Self {
        Permission {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl AuthzBackend for DbBackend {
    type Permission = Permission;
    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let permissions: Vec<Self::Permission> = sqlx::query_as(
            r#"
            select distinct permissions.name
            from users
            join users_groups on users.id = users_groups.user_id
            join groups_permissions on users_groups.group_id = groups_permissions.group_id
            join permissions on groups_permissions.permission_id = permissions.id
            where users.id = ?
            "#,
        )
        .bind(user.id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(permissions.into_iter().collect())
    }

    async fn has_perm(
        &self,
        user: &Self::User,
        perm: Self::Permission,
    ) -> Result<bool, Self::Error> {
        println!("{:?}", user.id);
        println!("{:?}", perm.name);
        let has_perm = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE id = $1 AND role = $2
            )
            "#,
        )
        .bind(user.id)
        .bind(&perm.name)
        .fetch_one(&*self.pool)
        .await?;

        Ok(has_perm)
    }
}
