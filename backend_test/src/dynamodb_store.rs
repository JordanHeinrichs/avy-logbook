use async_trait::async_trait;
use aws_sdk_dynamodb::primitives::Blob;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use rmp_serde;
use tower_sessions::{session::Id, Session, SessionStore};

/// An error type for `DynamoDbStore`.
#[derive(thiserror::Error, Debug)]
pub enum DynamoDbStoreError {
    /// A variant to map to `mongodb::error::Error` errors.
    #[error("DynamoDb error: {0}")]
    DynamoDb(#[from] Error),

    /// A variant to map `rmp_serde` encode errors.
    #[error("Rust MsgPack encode error: {0}")]
    RmpSerdeEncode(#[from] rmp_serde::encode::Error),

    /// A variant to map `rmp_serde` decode errors.
    #[error("Rust MsgPack decode error: {0}")]
    RmpSerdeDecode(#[from] rmp_serde::decode::Error),
}

/// A DynamoDB session store.
#[derive(Clone, Debug)]
pub struct DynamoDbStore {
    client: Client,
    table: String,
}

impl DynamoDbStore {
    pub fn new(client: Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl SessionStore for DynamoDbStore {
    type Error = DynamoDbStoreError;
    async fn save(&self, session: &Session) -> Result<(), Self::Error> {
        let session_id = AttributeValue::S(session.id().to_string());
        let serialized_session = rmp_serde::to_vec(&session).unwrap();
        let session_data = AttributeValue::B(Blob::new(serialized_session));

        self.client
            .put_item()
            .table_name(&self.table)
            .item("session_id", session_id)
            .item("session_data", session_data)
            .send()
            .await
            .map_err(|e| Error::from(e))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> Result<Option<Session>, Self::Error> {
        let result = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("session_id", AttributeValue::S(session_id.to_string()))
            .send()
            .await
            .map_err(|e| Error::from(e))?;

        if let Some(result) = result.item {
            let data = &result["session_data"];
            let data = data.as_b();
            let data = data.unwrap();
            let session: Session = rmp_serde::from_slice(data.as_ref())?;

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> Result<(), Self::Error> {
        self.client
            .delete_item()
            .key("session_id", AttributeValue::S(session_id.to_string()))
            .send()
            .await
            .map_err(|e| Error::from(e))?;

        Ok(())
    }
}
