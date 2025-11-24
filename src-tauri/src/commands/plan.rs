use anyhow::Result;

use crate::db::DatabaseState;
use crate::models::TripPlan;
use crate::AppError;

#[tauri::command]
pub async fn upsert_plan(
    plan: TripPlan,
    state: tauri::State<'_, DatabaseState>,
) -> Result<TripPlan, AppError> {
    let pool = &state.0;
    let insert_query = r#"
        INSERT INTO trip_plan (
            trip_id,
            areas_to_avoid,
            plan_left_with_someone,
            decision_points_considered,
            decision_points_comment
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5
        )
        ON CONFLICT (trip_id)
        DO UPDATE SET
            areas_to_avoid = $2,
            plan_left_with_someone = $3,
            decision_points_considered = $4,
            decision_points_comment = $5
        RETURNING *
    "#;

    let inserted = sqlx::query_as::<_, TripPlan>(insert_query)
        .bind(plan.trip_id)
        .bind(plan.areas_to_avoid)
        .bind(plan.plan_left_with_someone)
        .bind(plan.decision_points_considered)
        .bind(plan.decision_points_comment)
        .fetch_one(pool)
        .await?;
    Ok(inserted)
}

#[tauri::command]
pub async fn fetch_plan(
    trip_id: i64,
    state: tauri::State<'_, DatabaseState>,
) -> Result<TripPlan, AppError> {
    let pool = &state.0;

    let fetched = sqlx::query_as::<_, TripPlan>(
        r#"
        SELECT
            trip_id,
            areas_to_avoid,
            plan_left_with_someone,
            decision_points_considered,
            decision_points_comment
        FROM trip_plan
        WHERE trip_id = $1
        "#,
    )
    .bind(trip_id)
    .fetch_one(pool)
    .await?;
    Ok(fetched)
}
