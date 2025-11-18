use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::AvyObservation;
use crate::AppError;

#[tauri::command]
pub async fn create_avy_observation(
    avy_observation: AvyObservation,
    state: tauri::State<'_, DatabaseState>,
) -> Result<AvyObservation, AppError> {
    let pool = &state.0;
    let insert_query = r#"
        INSERT INTO avy_observation (
            trip_id,
            observation_time,
            avy_activity_size,
            avy_activity_trigger,
            avy_activity_characteristic,
            instability_see_feel,
            instability_ct,
            instability_ect,
            comment)
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9
        )
        RETURNING *
    "#;

    let inserted = sqlx::query_as::<_, AvyObservation>(insert_query)
        .bind(avy_observation.trip_id)
        .bind(avy_observation.observation_time)
        .bind(avy_observation.avy_activity_size)
        .bind(avy_observation.avy_activity_trigger)
        .bind(avy_observation.avy_activity_characteristic)
        .bind(avy_observation.instability_see_feel)
        .bind(avy_observation.instability_ct)
        .bind(avy_observation.instability_ect)
        .bind(avy_observation.comment)
        .fetch_one(pool)
        .await?;
    Ok(inserted)
}

#[tauri::command]
pub async fn fetch_avy_observations(
    trip_id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Vec<AvyObservation>, AppError> {
    let pool = &state.0;

    let fetched = sqlx::query_as::<_, AvyObservation>(
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
    .bind(trip_id)
    .fetch_all(pool)
    .await?;

    Ok(fetched)
}

#[tauri::command]
pub async fn edit_avy_observation(
    avy_observation: AvyObservation,
    state: tauri::State<'_, DatabaseState>,
) -> Result<AvyObservation, AppError> {
    let pool = &state.0;

    let updated = sqlx::query_as::<_, AvyObservation>(
        r#"
        UPDATE avy_observation SET
            avy_activity_size = $1,
            avy_activity_trigger = $2,
            avy_activity_characteristic = $3,
            instability_see_feel = $4,
            instability_ct = $5,
            instability_ect = $6
            comment = $7
        WHERE
            trip_id = $8
            AND observation_time = $9
        RETURNING *
        "#,
    )
    .bind(avy_observation.avy_activity_size)
    .bind(avy_observation.avy_activity_trigger)
    .bind(avy_observation.avy_activity_characteristic)
    .bind(avy_observation.instability_see_feel)
    .bind(avy_observation.instability_ct)
    .bind(avy_observation.instability_ect)
    .bind(avy_observation.comment)
    .bind(avy_observation.trip_id)
    .bind(avy_observation.observation_time)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}
