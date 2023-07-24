use axum::{response::IntoResponse, Json};
use axum_sessions::async_session::serde_json::json;
use serde::Deserialize;

/// route to handle log in
#[allow(clippy::unused_async)]
#[allow(clippy::missing_panics_doc)]
pub async fn login(Json(login): Json<Login>) -> impl IntoResponse {
    tracing::info!("Logging in user: {}", login.username);
    // This will be the hit from the page that Strava redirects to but for now assume all users are valid and just create JWT

    Json(json!({"result": "ok"}))
}

/// route to handle log out
#[allow(clippy::unused_async)]
pub async fn logout() -> impl IntoResponse {
    tracing::info!("Logging out user");
    Json(json!({"result": "ok"}))
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}
