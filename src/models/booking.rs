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
