use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::path::PathBuf;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_dir: &PathBuf) -> Result<Self> {
        let db_path = app_dir.join("avy_logbook.db");

        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .pragma("foreign_keys", "ON")
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        // Create and initialize the database pool
        let pool = SqlitePool::connect_with(connection_options).await?;

        // Run migrations regardless of whether the database is new
        // SQLx will track which migrations have been run
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

// State management for Tauri
#[allow(dead_code)]
pub struct DatabaseState(pub Pool<Sqlite>);
