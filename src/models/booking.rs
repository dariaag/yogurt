use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use sqlx::FromRow;

use crate::utils::serialize_dt::serialize_dt;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Booking {
    booking_id: i32,
    class_id: i32,
    user_id: i32,
    #[serde(serialize_with = "serialize_dt")]
    created_at: DateTime<Utc>,
    status: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewBooking {
    class_id: i32,
    user_id: i32,
    status: String,
}

impl Booking {
    pub async fn create(
        new_booking: NewBooking,
        pool: &sqlx::PgPool,
    ) -> Result<Booking, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let booking = sqlx::query_as::<_, Booking>("INSERT INTO bookings (class_id, user_id, status) VALUES ($1, $2, $3) RETURNING booking_id, class_id, user_id, created_at, status")
            .bind(&new_booking.class_id)
            .bind(&new_booking.user_id)
            .bind(&new_booking.status)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(booking)
    }

    pub async fn find_by_id(id: i32, pool: &sqlx::PgPool) -> Result<Booking, sqlx::Error> {
        let booking = sqlx::query_as::<_, Booking>("SELECT * FROM bookings WHERE booking_id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(booking)
    }

    pub async fn find_all(pool: &sqlx::PgPool) -> Result<Vec<Booking>, sqlx::Error> {
        let bookings = sqlx::query_as::<_, Booking>("SELECT * FROM bookings")
            .fetch_all(pool)
            .await?;
        Ok(bookings)
    }

    pub async fn find_by_user(
        user_id: i32,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Booking>, sqlx::Error> {
        let bookings = sqlx::query_as::<_, Booking>("SELECT * FROM bookings WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(bookings)
    }

    pub async fn delete(id: i32, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM bookings WHERE booking_id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_all(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM bookings").execute(pool).await?;
        Ok(())
    }
}
