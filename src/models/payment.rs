use crate::utils::serialize_dt::serialize_dt;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Payment {
    payment_id: i32,
    booking_id: i32,
    amount: f64,
    payment_method: String,
    status: String,
    #[serde(serialize_with = "serialize_dt")]
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewPayment {
    booking_id: i32,
    amount: f64,
    payment_method: String,
    status: String,
}

impl Payment {
    pub async fn create(new_payment: NewPayment, pool: &PgPool) -> Result<Payment, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let payment = sqlx::query_as::<_, Payment>("INSERT INTO payments (booking_id, amount, payment_method, status) VALUES ($1, $2, $3, $4) RETURNING payment_id, booking_id, amount, payment_method, status, created_at")
            .bind(&new_payment.booking_id)
            .bind(&new_payment.amount)
            .bind(&new_payment.payment_method)
            .bind(&new_payment.status)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(payment)
    }
}
