use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::{AvalancheForecast, AvalancheProblem, Elevation, ProblemType};
use crate::AppError;

#[tauri::command]
pub async fn edit_avy_forecast(
    forecast: AvalancheForecast,
    state: tauri::State<'_, DatabaseState>,
) -> Result<AvalancheForecast, AppError> {
    let pool = &state.0;
    let insert_query = r#"
        UPDATE avalanche_forecast SET
            forecast_alp = $1,
            forecast_tl = $2,
            forecast_btl = $3,
            macro_trends = $4,
            confidence = $5,
            comment = $6
        WHERE
            id = $7
        RETURNING *
    "#;

    let updated = sqlx::query_as::<_, AvalancheForecast>(insert_query)
        .bind(forecast.forecast_alp)
        .bind(forecast.forecast_tl)
        .bind(forecast.forecast_btl)
        .bind(forecast.macro_trends)
        .bind(forecast.confidence)
        .bind(forecast.comment)
        .bind(forecast.id)
        .fetch_one(pool)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn fetch_avy_forecast(
    trip_id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<AvalancheForecast, AppError> {
    let pool = &state.0;
    let avy_forecast = sqlx::query_as::<_, AvalancheForecast>(
        r#"
        SELECT
            id,
            trip_id,
            forecast_alp,
            forecast_tl,
            forecast_btl,
            macro_trends,
            confidence,
            comment
        FROM avalanche_forecast
        WHERE trip_id = ?
        "#,
    )
    .bind(trip_id)
    .fetch_optional(pool)
    .await?;

    match avy_forecast {
        Some(avy_forecast) => return Ok(avy_forecast),
        None => {
            let result = sqlx::query(
                r#"
                INSERT INTO avalanche_forecast (trip_id)
                VALUES (?)
                "#,
            )
            .bind(trip_id)
            .execute(pool)
            .await?;

            let id = result.last_insert_rowid();
            let new_forecast = AvalancheForecast {
                id,
                trip_id,
                forecast_alp: None,
                forecast_tl: None,
                forecast_btl: None,
                macro_trends: None,
                confidence: None,
                comment: None,
            };
            return Ok(new_forecast);
        }
    }
}

#[tauri::command]
pub async fn fetch_avy_problems(
    forecast_id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Vec<AvalancheProblem>, AppError> {
    let pool = &state.0;
    let avy_problems = sqlx::query_as::<_, AvalancheProblem>(
        r#"
        SELECT
            id,
            forecast_id,
            elevation,
            problem_type
        FROM avalanche_problem
        WHERE forecast_id = ?
        "#,
    )
    .bind(forecast_id)
    .fetch_all(pool)
    .await?;

    Ok(avy_problems)
}

#[tauri::command]
pub async fn create_avy_problem(
    forecast_id: i64,
    elevation: Elevation,
    problem_type: ProblemType,
    state: tauri::State<'_, DatabaseState>,
) -> Result<AvalancheProblem, AppError> {
    let pool = &state.0;

    let avy_problems = sqlx::query_as::<_, AvalancheProblem>(
        r#"
        INSERT INTO avalanche_problem (
            forecast_id,
            elevation,
            problem_type
        )
        VALUES (
            $1,
            $2,
            $3
        )
        RETURNING *
        "#,
    )
    .bind(forecast_id)
    .bind(elevation)
    .bind(problem_type)
    .fetch_one(pool)
    .await?;

    Ok(avy_problems)
}

#[tauri::command]
pub async fn delete_avy_problem(
    id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<bool, AppError> {
    let pool = &state.0;

    sqlx::query(
        r#"
        DELETE FROM avalanche_problem WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(true)
}
