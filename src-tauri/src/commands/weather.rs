use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::{NewWeather, Weather};
use crate::AppError;

#[tauri::command]
pub async fn create_weather(
    weather: NewWeather,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Weather, AppError> {
    let pool = &state.0;
    let insert_query = r#"
        INSERT INTO weather (
            trip_id,
            observation_time,
            precipitation,
            accumulation,
            wind_speed,
            wind_direction,
            solar_radiation,
            comment)
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8
        )
        RETURNING *
    "#;

    let inserted = sqlx::query_as::<_, Weather>(insert_query)
        .bind(weather.trip_id)
        .bind(weather.observation_time)
        .bind(weather.precipitation)
        .bind(weather.accumulation)
        .bind(weather.wind_speed)
        .bind(weather.wind_direction)
        .bind(weather.solar_radiation)
        .bind(weather.comment)
        .fetch_one(pool)
        .await?;
    Ok(inserted)
}

#[tauri::command]
pub async fn fetch_weather(
    id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Weather, AppError> {
    let pool = &state.0;

    let query = r#"
        SELECT
            id,
            trip_id,
            observation_time,
            precipitation,
            accumulation,
            wind_speed,
            wind_direction,
            solar_radiation,
            comment
        FROM weather
        WHERE id = $1
        "#;
    let fetched = sqlx::query_as::<_, Weather>(query)
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fetched)
}

#[tauri::command]
pub async fn edit_weather(
    weather: Weather,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Weather, AppError> {
    let pool = &state.0;

    let query = r#"
        UPDATE weather SET
            precipitation = $1,
            accumulation = $2,
            wind_speed = $3,
            wind_direction = $4,
            solar_radiation = $5,
            comment = $6,
            observation_time = $7
        WHERE
            id = $8
        RETURNING *
        "#;

    let updated = sqlx::query_as::<_, Weather>(query)
        .bind(weather.precipitation)
        .bind(weather.accumulation)
        .bind(weather.wind_speed)
        .bind(weather.wind_direction)
        .bind(weather.solar_radiation)
        .bind(weather.comment)
        .bind(weather.observation_time)
        .bind(weather.id)
        .fetch_one(pool)
        .await?;
    Ok(updated)
}
