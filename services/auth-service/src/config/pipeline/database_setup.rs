use sqlx::{Pool, Postgres};
use std::time::Duration;

pub async fn create_database_pool(
    config: &crate::infrastructure::config::AppConfig,
) -> Result<Pool<Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(config.database.connect_timeout))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout))
        .connect(&config.database.url)
        .await
}
