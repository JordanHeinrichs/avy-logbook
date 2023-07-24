use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::{db_client::DbClient, middlewares, routes};

// ********
// BACK END
// ********
// Back end server built form various routes that are either public, require auth, or secure login
pub fn backend(db_client: Arc<DbClient>) -> Router {
    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new()
        .route("/api/test", get(routes::api::test_handler))
        .route_layer(middleware::from_fn_with_state(
            db_client.clone(),
            middlewares::auth,
        ))
        .route("/api/login", post(routes::login)) // sets username in session
        .route("/api/logout", get(routes::logout)) // deletes username in session
        .with_state(db_client)
}
