use aws_sdk_dynamodb::Client;

#[derive(Clone, Debug)]
pub struct DynamoDbState {
    pub client: Client,
    pub table: String,
}

impl DynamoDbState {
    pub fn new(client: Client, table: String) -> Self {
        Self { client, table }
    }
}
