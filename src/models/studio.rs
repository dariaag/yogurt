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

impl Studio {
    pub async fn create(studio: Studio, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO studios (owner_id, name, description, address, phone) VALUES ($1, $2, $3, $4, $5)")
            .bind(studio.owner_id)
            .bind(studio.name)
            .bind(studio.description)
            .bind(studio.address)
            .bind(studio.phone)
            .execute(pool)
            .await?;
        Ok(())
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
}
