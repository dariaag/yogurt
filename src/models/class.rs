use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::serialize_dt::serialize_dt;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Class {
    class_id: i32,
    studio_id: i32,
    title: String,
    description: Option<String>,
    instructor_name: Option<String>,
    start_time: NaiveDateTime,
    duration: i32,
    capacity: i32,
    price: f64,
    #[serde(serialize_with = "serialize_dt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewClass {
    studio_id: i32,
    title: String,
    description: Option<String>,
    instructor_name: Option<String>,
    start_time: NaiveDateTime,
    duration: i32,
    capacity: i32,
    price: f64,
}

impl Class {
    pub async fn create(class: NewClass, pool: &sqlx::PgPool) -> Result<Class, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let class = sqlx::query_as::<_, Class>("INSERT INTO classes (studio_id, title, description, instructor_name, start_time, duration, capacity, price) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING class_id, studio_id, title, description, instructor_name, start_time, duration, capacity, price, created_at")
            .bind(&class.studio_id)
            .bind(&class.title)
            .bind(&class.description)
            .bind(&class.instructor_name)
            .bind(&class.start_time)
            .bind(&class.duration)
            .bind(&class.capacity)
            .bind(&class.price)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(class)
    }

    pub async fn find_by_id(id: i32, pool: &sqlx::PgPool) -> Result<Class, sqlx::Error> {
        let class = sqlx::query_as::<_, Class>("SELECT * FROM classes WHERE class_id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(class)
    }

    pub async fn find_all(pool: &sqlx::PgPool) -> Result<Vec<Class>, sqlx::Error> {
        let classes = sqlx::query_as::<_, Class>("SELECT * FROM classes")
            .fetch_all(pool)
            .await?;
        Ok(classes)
    }

    pub async fn find_by_studio(
        studio_id: i32,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Class>, sqlx::Error> {
        let classes = sqlx::query_as::<_, Class>("SELECT * FROM classes WHERE studio_id = $1")
            .bind(studio_id)
            .fetch_all(pool)
            .await?;
        Ok(classes)
    }

    pub async fn delete(id: i32, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM classes WHERE class_id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_all(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM classes").execute(pool).await?;
        Ok(())
    }
}
