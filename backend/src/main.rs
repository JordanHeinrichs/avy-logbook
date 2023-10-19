#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use axum::Router;
use std::env;
use std::net::SocketAddr;
use tracing::log::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod middlewares;
pub mod routes;
mod services;
mod shared_state;

// SETUP Constants
const SERVER_PORT: &str = "8080";
const SERVER_HOST: &str = "0.0.0.0";

/// Server that is split into a Frontend to serve static files (Svelte) and Backend
/// Backend is further split into a non authorized area and a secure area
/// The Back end is using 2 middleware: sessions (managing session data)
#[tokio::main]
async fn main() {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "svelte_axum_project=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // configure server from environmental variables
    let config = from_env();

    let addr: SocketAddr = format!("{}:{}", config.db_server_host, config.db_server_port)
        .parse()
        .expect("Can not parse address and port");

    let db_config = shared_state::SharedState::get_config().await;

    // create store for backend.  Stores an api_token.
    let shared_state = shared_state::SharedState::new(&db_config, config.jwt_secret);

    // combine the front and backend into server
    let app = Router::new().merge(services::backend(shared_state));

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}

// Variables from Environment or default to configure server
// port, host, secret
fn from_env() -> Config {
    if env::var("SERVER_SECRET").is_err() {
        warn!("env var SERVER_SECRET should be set and unique (64 bytes long)");
    }
    return Config {
        db_server_port: env::var("SERVER_PORT")
            .ok()
            .unwrap_or_else(|| SERVER_PORT.to_string()),
        db_server_host: env::var("SERVER_HOST")
            .ok()
            .unwrap_or_else(|| SERVER_HOST.to_string()),
        jwt_secret: env::var("JWT_SECRET")
            .ok()
            .unwrap_or_else(|| SERVER_HOST.to_string()),
    };
}

pub struct Config {
    db_server_port: String,
    db_server_host: String,
    jwt_secret: String,
}
