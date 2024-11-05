use tower_sessions_dynamodb_store::{DynamoDBStoreKey, DynamoDBStoreProps};

use crate::database::setup::DynamoDbClient;

// Minimal Redis configuration.
pub fn session_store_config(client: &DynamoDbClient) -> DynamoDBStoreProps {
    DynamoDBStoreProps {
        table_name: client.table.clone(),
        partition_key: DynamoDBStoreKey {
            name: "PK".to_string(),
            prefix: Some("user_id::".to_string()),
            suffix: None,
        },
        sort_key: Some(DynamoDBStoreKey {
            name: "SK".to_string(),
            prefix: Some("session_id::".to_string()),
            suffix: None,
        }),
        expirey_name: "expire_at".to_string(),
        data_name: "data".to_string(),
        create_key_max_retry_attempts: 5,
    }
}
