mod config;
mod dynamodb_store;

use aws_config::BehaviorVersion;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dynamodb_store::DynamoDbState;
use lambda_http::{run, Error};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct AppState {
    db: DynamoDbState,
}

const TABLE_NAME: &str = "AvyLogbook";

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

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_url("http://localhost:8000")
        .build();
    let client = aws_sdk_dynamodb::Client::from_conf(config);

    let db_state = DynamoDbState::new(client, String::from(TABLE_NAME));
    let app_state = Arc::new(AppState { db: db_state });

    // build our application with a route
    let app = Router::new()
        .route("/hello_world", post(hello_someone))
        .route("/hello_world", get(hello_world))
        .route("/login", get(login))
        .with_state(app_state);

    run(app).await
}

async fn hello_someone(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HelloWorldRequest>,
) -> (StatusCode, Json<HelloWorldResponse>) {
    let db = &state.as_ref().db;
    db.client
        .put_item()
        .table_name(&db.table)
        .item(
            "PK",
            aws_sdk_dynamodb::types::AttributeValue::S(payload.first_name.clone()),
        )
        .item(
            "SK",
            aws_sdk_dynamodb::types::AttributeValue::S(
                payload.last_name.clone().unwrap_or("not provided".into()),
            ),
        )
        .send()
        .await
        .unwrap();
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

async fn hello_world(State(state): State<Arc<AppState>>) -> (StatusCode, Json<HelloWorldResponse>) {
    let res = HelloWorldResponse {
        result: format!("Hello world"),
    };
    (StatusCode::OK, Json(res))
}

async fn login() -> (StatusCode, Json<HelloWorldResponse>) {
    // In the future this will instead be the callback from OAuth, for now
    // just create a session
    let res: HelloWorldResponse = HelloWorldResponse {
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

// pub async fn session_middleware(
//     State(state): State<Arc<AppState>>,
//     request: Request,
//     next: Next,
// ) -> Response {
//     let result: aws_sdk_dynamodb::operation::get_item::GetItemOutput = self
//         .client
//         .get_item()
//         .table_name(&self.table)
//         .key("session_id", AttributeValue::S(session_id.to_string()))
//         .send()
//         .await
//         .map_err(|e| Error::from(e))?;

//     if let Some(result) = result.item {
//         let data = &result["session_data"];
//         let data = data.as_b();
//         let data = data.unwrap();
//         let session: Session = rmp_serde::from_slice(data.as_ref())?;

//         Ok(Some(session))
//     } else {
//         Ok(None)
//     }
// }

// pub async fn create_session(request: Request, next: Next) -> Response {
//     let session_id = AttributeValue::S(session.id().to_string());
//     let serialized_session = rmp_serde::to_vec(&session).unwrap();
//     let session_data = AttributeValue::B(Blob::new(serialized_session));

//     self.client
//         .put_item()
//         .table_name(&self.table)
//         .item("session_id", session_id)
//         .item("session_data", session_data)
//         .send()
//         .await
//         .map_err(|e| Error::from(e))?;

//     Ok(())
// }
