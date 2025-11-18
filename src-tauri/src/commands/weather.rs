use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::Weather;
use crate::AppError;

#[tauri::command]
pub async fn create_weather(
    weather: Weather,
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
    trip_id: i64,
    observation_time: Option<String>,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Weather, AppError> {
    let pool = &state.0;

    let query = r#"
        SELECT
            trip_id,
            observation_time,
            precipitation,
            accumulation,
            wind_speed,
            wind_direction,
            solar_radiation,
            comment
        FROM weather
        WHERE trip_id = $1
        "#;

    let fetched = match observation_time {
        Some(observation_time) => {
            let query = format!("{} AND observation_time = $2", query);
            sqlx::query_as::<_, Weather>(query.as_str())
                .bind(trip_id)
                .bind(observation_time)
                .fetch_one(pool)
                .await?
        }
        None => {
            let query = format!("{} AND observation_time IS NULL", query);
            sqlx::query_as::<_, Weather>(query.as_str())
                .bind(trip_id)
                .fetch_one(pool)
                .await?
        }
    };
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
            comment = $6
        WHERE
            trip_id = $7
        "#;

    let updated = match weather.observation_time {
        Some(observation_time) => {
            let query = format!("{} AND observation_time = $8 RETURNING *", query);
            sqlx::query_as::<_, Weather>(query.as_str())
                .bind(weather.precipitation)
                .bind(weather.accumulation)
                .bind(weather.wind_speed)
                .bind(weather.wind_direction)
                .bind(weather.solar_radiation)
                .bind(weather.comment)
                .bind(weather.trip_id)
                .bind(observation_time)
                .fetch_one(pool)
                .await?
        }
        None => {
            let query = format!("{} AND observation_time IS NULL RETURNING *", query);
            sqlx::query_as::<_, Weather>(query.as_str())
                .bind(weather.precipitation)
                .bind(weather.accumulation)
                .bind(weather.wind_speed)
                .bind(weather.wind_direction)
                .bind(weather.solar_radiation)
                .bind(weather.comment)
                .bind(weather.trip_id)
                .fetch_one(pool)
                .await?
        }
    };
    Ok(updated)
}
