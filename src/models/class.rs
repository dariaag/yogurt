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
