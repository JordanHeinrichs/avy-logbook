use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Json, Router,
};
use lambda_http::{run, Error};
mod dynamodb_store;
use self::dynamodb_store::DynamoDbStore;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_sessions::SessionManagerLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    let session_store = DynamoDbStore::new(client, String::from("avy_logbook"));
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(SessionManagerLayer::new(session_store).with_secure(true));

    // build our application with a route
    let app = Router::new()
        .route("/hello_world", post(hello_someone))
        .route("/hello_world", get(hello_world))
        .layer(session_service)
        .route("/login", get(login));

    run(app).await
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

async fn login() -> (StatusCode, Json<HelloWorldResponse>) {
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
