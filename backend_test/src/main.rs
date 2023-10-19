use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/hello_world", post(hello_someone))
        .route("/hello_world", get(hello_world));

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_someone(
    Json(payload): Json<HelloWorldRequest>,
) -> (StatusCode, Json<HelloWorldResponse>) {
    let res = match &payload.last_name {
        Some(last_name) => HelloWorldResponse {
            result: format!("Hello {} {}", payload.first_name, last_name),
        },
        None => HelloWorldResponse {
            result: format!("Hello {}", payload.first_name,),
        },
    };
    (StatusCode::OK, Json(res))
}

async fn hello_world() -> (StatusCode, Json<HelloWorldResponse>) {
    let res = HelloWorldResponse {
        result: format!("Hello world"),
    };
    (StatusCode::OK, Json(res))
}

#[derive(Deserialize)]
struct HelloWorldRequest {
    first_name: String,
    last_name: Option<String>,
}

#[derive(Serialize)]
struct HelloWorldResponse {
    result: String,
}
