use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::{
    AvalancheForecast, AvalancheProblem, AvyObservation, FullTripDetails, Trip, TripPlan, Weather,
};
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

#[tauri::command]
pub async fn fetch_full_trip(
    id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<FullTripDetails, AppError> {
    let pool = &state.0;
    let trip = sqlx::query_as::<_, Trip>("SELECT id, name, trip_date FROM trip WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

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
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let weather = sqlx::query_as::<_, Weather>(
        r#"
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
        WHERE trip_id = ?
        SORT BY
            observation_time ASC NULLS FIRST
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let forecast_problems = match &avy_forecast {
        Some(forecast) => {
            sqlx::query_as::<_, AvalancheProblem>(
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
            .bind(forecast.id.clone())
            .fetch_all(pool)
            .await?
        }
        None => vec![],
    };

    let plan = sqlx::query_as::<_, TripPlan>(
        r#"
        SELECT
            trip_id,
            areas_to_avoid,
            plan_left_with_someone,
            decision_points_considered,
            decision_points_comment
        FROM trip_plan
        WHERE trip_id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let avy_observations = sqlx::query_as::<_, AvyObservation>(
        r#"
        SELECT
            trip_id
            observation_time,
            avy_activity_size,
            avy_activity_trigger,
            avy_activity_characteristic,
            instability_see_feel,
            instability_ct,
            instability_ect,
            comment
        FROM avy_observation
        WHERE
            trip_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(FullTripDetails {
        trip,
        forecast: avy_forecast,
        forecast_problems,
        plan,
        weather_observations: weather,
        avy_observations,
    })
}
