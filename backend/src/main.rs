use anyhow::Result;
use api::{private, public};
use auth::{backend::AuthBackend, sessions::session_store_config};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_login::AuthManagerLayerBuilder;
use database::setup::init_dynamo_client;
use http::HeaderValue;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, limit::RequestBodyLimitLayer,
    services::ServeDir,
};
use tower_sessions::{
    cookie::{Key, SameSite},
    Expiry, SessionManagerLayer,
};
use tower_sessions_dynamodb_store::DynamoDBStore;
use utils::shutdown_signal;

mod api;
mod auth;
mod database;
mod errors;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    tracing_subscriber::fmt::init();

    tracing::info!("Starting server.");

    // PostgreSQL connection pool.
    let dynamo_client = init_dynamo_client().await?;

    // Session layer as a request extension using Redis as a session store.
    let session_store =
        DynamoDBStore::new(dynamo_client.client, session_store_config(&dynamo_client));

    // Generates a singing/encryption key for the cookies.
    let key = Key::generate();

    // Session layer as a request extension.
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(time::Duration::minutes(30)))
        .with_same_site(SameSite::Strict)
        .with_secure(true)
        .with_private(key);

    // Authentication backend.
    let backend = AuthBackend::new(&db_pool);

    // Authentication layer.
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // Add CORS header to responses.
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:8000".parse::<HeaderValue>()?)
        .max_age(Duration::from_secs(3600));

    // Compresses response bodies.
    let compression = CompressionLayer::new();

    // Limit the size of request bodies to 1 MB.
    let request_size = RequestBodyLimitLayer::new(1024 * 1024);

    // Routes that require authentication.
    // The custom middleware runs before these handlers.
    let protected_routes = Router::new()
        .route("/protected", get(private::example_protected_handler))
        .route("/logout", post(private::logout_handler))
        .route_layer(middleware::from_fn(api::middleware::check_auth));

    // Routes that are public.
    let public_routes = Router::new()
        .route("/", get(public::example_endpoint_handler))
        .route("/start", get(public::start_handler))
        .route("/login", post(public::login_handler))
        .route("/register", post(public::register_handler));

    // Serve the frontend statically in production.
    let static_dir = ServeDir::new("frontend/dist");

    // Router application.
    let app = Router::new()
        .nest("/api/", public_routes)
        .nest("/api/", protected_routes)
        .nest_service("/", static_dir)
        .layer(request_size)
        .layer(auth_layer)
        .layer(cors)
        .layer(compression)
        .with_state(dynamo_client);

    // Socket address.
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    // TCP listener.
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;

    tracing::info!("Backend listening on: {}", socket_addr);

    // Serve the application with hyper.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    // Ensure the task managing the Redis connections runs to completion before exiting.
    redis_connection.await??;

    Ok(())
}
