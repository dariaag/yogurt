use crate::config;
use sqlx::postgres::PgPool;

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = config::get_database_url();
    PgPool::connect(&database_url).await
}
