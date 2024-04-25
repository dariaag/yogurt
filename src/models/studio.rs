use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::serialize_dt::serialize_dt;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Studio {
    id: i32,
    owner_id: i32,
    name: String,
    description: Option<String>,
    address: Option<String>,
    phone: Option<String>,
    #[serde(serialize_with = "serialize_dt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewStudio {
    owner_id: i32,
    name: String,
    description: Option<String>,
    address: Option<String>,
    phone: Option<String>,
}

impl Studio {
    pub async fn create(new_studio: NewStudio, pool: &PgPool) -> Result<Studio, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let studio = sqlx::query_as::<_, Studio>("INSERT INTO studios (owner_id, name, description, address, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id, owner_id, name, description, address, phone, created_at")
            .bind(&new_studio.owner_id)
            .bind(&new_studio.name)
            .bind(&new_studio.description)
            .bind(&new_studio.address)
            .bind(&new_studio.phone)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(studio)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Studio, sqlx::Error> {
        let studio = sqlx::query_as::<_, Studio>("SELECT * FROM studios WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(studio)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Studio>, sqlx::Error> {
        let studios = sqlx::query_as::<_, Studio>("SELECT * FROM studios")
            .fetch_all(pool)
            .await?;
        Ok(studios)
    }

    pub async fn find_by_owner(owner_id: i32, pool: &PgPool) -> Result<Vec<Studio>, sqlx::Error> {
        let studios = sqlx::query_as::<_, Studio>("SELECT * FROM studios WHERE owner_id = $1")
            .bind(owner_id)
            .fetch_all(pool)
            .await?;
        Ok(studios)
    }
}
