use crate::{
    database::{models::user::User, setup::DynamoDbClient},
    errors::AuthError,
};
use anyhow::Result;
use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use serde::Deserialize;
use tokio::task;

/// AuthSession with Backend type alias.
pub type AuthSession = axum_login::AuthSession<AuthBackend>;

/// Credentials used for authentication.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Backend used for authentication with PostgreSQL database pool.
#[derive(Debug, Clone)]
pub struct AuthBackend {
    pub dynamo_client: DynamoDbClient,
}

impl AuthBackend {
    /// Create a new backend instance.
    pub fn new(dynamo_client: &DynamoDbClient) -> Self {
        Self {
            dynamo_client: dynamo_client.clone(),
        }
    }

    /// Verify a password against a hash.
    fn check_password(password: String, hash: &str) -> Result<bool, AuthError> {
        verify_password(password, hash)
            .map(|_| true)
            .map_err(|_| AuthError::InvalidPassword)
    }

    /// Fetch a user by username from the database.
    async fn fetch_user_by_username(&self, username: &str) -> Result<User, AuthError> {
        self.dynamo_client
            .client
            .put_item()
            .table_name(&self.dynamo_client.table)
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

        let db_connection = self.dynamo_client.get().await?;
        let row = db_connection
            .query_one("SELECT * FROM users WHERE username = $1", &[&username])
            .await?;
        Ok(row.into())
    }

    /// Fetch a user by id from the database.
    async fn fetch_user_by_id(&self, user_id: &UserId<Self>) -> Result<User, AuthError> {
        let db_connection = self.dynamo_client.get().await?;
        let row = db_connection
            .query_one("SELECT * FROM users WHERE id = $1", &[&user_id])
            .await?;
        Ok(row.into())
    }
}

/// Backend can authenticate users.
#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    /// Authenticates a user with the provided credentials.
    async fn authenticate(&self, creds: Credentials) -> Result<Option<Self::User>, Self::Error> {
        let user = self.fetch_user_by_username(&creds.username).await?;

        tracing::info!("A user is authenticating: {}.", user.username);

        // Offload the password verification to a blocking task.
        task::spawn_blocking(|| {
            Self::check_password(creds.password, &user.password_hash).map(|_| Some(user))
        })
        .await?
    }

    /// Retrieves a user by ID from the database.
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        self.fetch_user_by_id(user_id).await.map(Some)
    }
}
