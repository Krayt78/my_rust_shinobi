//! Database connection pool management

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database connection pool type alias
pub type DbPool = PgPool;

/// Initialize the database connection pool
/// 
/// # Arguments
/// * `database_url` - PostgreSQL connection string
/// 
/// # Returns
/// * `Result<DbPool, sqlx::Error>` - Connection pool or error
pub async fn init_db_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await
}

/// Get the database URL from environment variables
pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file")
}

/// Run database migrations
/// 
/// This should be called on application startup to ensure
/// the database schema is up to date.
pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}

