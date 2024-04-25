use crate::utils::serialize_dt::serialize_dt;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
