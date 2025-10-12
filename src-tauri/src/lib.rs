use serde::Serialize;
use std::fs;
use tauri::Manager;
use thiserror::Error;

mod commands;
mod db;
pub mod models;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("An unexpected error occurred: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let error_message = self.to_string();
        serializer.serialize_str(&error_message)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the platform-specific application data directory path
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory.");

            // Check if the directory exists, and if not, create it
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir)
                    .expect("Failed to create application data directory.");
            }

            println!(
                "App data directory is {}",
                app_data_dir.as_os_str().to_str().unwrap()
            );

            tauri::async_runtime::block_on(async move {
                let database = db::Database::new(&app_data_dir)
                    .await
                    .expect("failed to initialize database");

                // Store database pool in app state
                app.manage(db::DatabaseState(database.pool));
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::trip_list,
            commands::create_trip,
            commands::fetch_trip,
            commands::fetch_full_trip,
            commands::fetch_avy_forecast,
            commands::edit_avy_forecast,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
