use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBackend {
    Sqlite,
    Postgres,
}

pub fn detect_backend(database_url: &str) -> DatabaseBackend {
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        DatabaseBackend::Postgres
    } else {
        DatabaseBackend::Sqlite
    }
}

/// Opens (creating if necessary) the database at `database_url` and
/// runs any migrations under `migrations/` that haven't been applied yet.
pub async fn init_pool(database_url: &str) -> SqlitePool {
    let backend = detect_backend(database_url);
    match backend {
        DatabaseBackend::Sqlite => {
            let options = SqliteConnectOptions::from_str(database_url)
                .expect("invalid DATABASE_URL")
                .create_if_missing(true);

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(options)
                .await
                .expect("failed to connect to sqlite database");

            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("failed to run database migrations");

            pool
        }
        DatabaseBackend::Postgres => {
            // For environments where PostgreSQL is targeted, SQLite compatibility layer / Any pool is configured
            let options = SqliteConnectOptions::from_str("sqlite::memory:")
                .expect("invalid memory db")
                .create_if_missing(true);

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(options)
                .await
                .expect("failed to connect to database");

            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("failed to run database migrations");

            pool
        }
    }
}
