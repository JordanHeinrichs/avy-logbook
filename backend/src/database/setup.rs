use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;

const TABLE_NAME: &str = "AvyLogbook";

#[derive(Clone, Debug)]
pub struct DynamoDbClient {
    pub client: Client,
    pub table: String,
}

impl DynamoDbClient {
    pub fn new(client: Client, table: String) -> Self {
        Self { client, table }
    }
}

/// Setup the Dynamo store.
pub async fn init_dynamo_client() -> Result<DynamoDbClient> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_url("http://localhost:8000")
        .build();

    let client = aws_sdk_dynamodb::Client::from_conf(config);
    let db_state = DynamoDbClient::new(client, String::from(TABLE_NAME));

    Ok(db_state)
}
