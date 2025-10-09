use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::Trip;
use crate::AppError;

#[tauri::command]
pub async fn trip_list(state: tauri::State<'_, DatabaseState>) -> Result<Vec<Trip>, AppError> {
    let stmt = r#"
        SELECT
            id,
            name,
            trip_date
        FROM trip
        ORDER BY trip_date DESC
    "#;

    let query = sqlx::query_as::<_, Trip>(stmt);
    let pool = &state.0;
    let log_entries = query.fetch_all(pool).await?;
    Ok(log_entries)
}

#[tauri::command]
pub async fn create_trip(
    name: String,
    trip_date: String,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Trip, AppError> {
    // 1. --- Validation ---
    // It's best practice to validate inputs in the backend.
    let trimmed_name = name.trim();
    if trimmed_name.is_empty() {
        return Err(AppError::Validation(
            "Trip name cannot be empty.".to_string(),
        ));
    }

    let pool = &state.0;
    let insert_query = r#"
        INSERT INTO trip (name, trip_date)
        VALUES (?, ?)
    "#;

    let result = sqlx::query(insert_query)
        .bind(trimmed_name)
        .bind(&trip_date)
        .execute(pool)
        .await?;

    let id = result.last_insert_rowid();
    let new_trip = Trip {
        id,
        name: trimmed_name.to_string(),
        trip_date,
    };

    Ok(new_trip)
}

#[tauri::command]
pub async fn fetch_trip(id: i64, state: tauri::State<'_, DatabaseState>) -> Result<Trip, AppError> {
    let pool = &state.0;
    let trip = sqlx::query_as::<_, Trip>("SELECT id, name, trip_date FROM trip WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(trip)
}
