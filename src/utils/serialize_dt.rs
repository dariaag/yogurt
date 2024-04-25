use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};

pub fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%m/%d/%Y %H:%M")
        .to_string()
        .serialize(serializer)
}
