use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::shared_state::SharedState;

/// middleware function to authenticate authorization token
/// check store that contains token and see if it matches authorization header starting with "Bearer"
/// used example in axum docs on middleware <https://docs.rs/axum/latest/axum/middleware/index.html>
///
/// Returns Error's in JSON format.
#[allow(clippy::missing_errors_doc)]
pub async fn auth<B: Send + Sync>(
    State(state): State<SharedState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<JsonError>)> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header: &http::HeaderValue| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        tracing::debug!("Authorization header missing or malformed");
        return Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())));
    };

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.jwt_token.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = JsonError {
            error: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    // Verify that the user in the database
    // state.client.as_ref().query()

    tracing::debug!("Received Authorization Header: {}", token);

    Ok(next.run(req).await)
}

#[derive(Serialize, Deserialize)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub const fn new(error: String) -> Self {
        Self { error }
    }

    pub fn unauthorized() -> Self {
        Self {
            error: "Unauthorized".into(),
        }
    }

    pub fn internal() -> Self {
        Self {
            error: "Internal Server Error".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
