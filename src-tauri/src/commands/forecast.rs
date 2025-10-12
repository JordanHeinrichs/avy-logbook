use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::AvalancheForecast;
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
            comments = $6
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
        .bind(forecast.comments)
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
            comments
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
                comments: None,
            };
            return Ok(new_forecast);
        }
    }
}
