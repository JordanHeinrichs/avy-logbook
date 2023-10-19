use std::sync::Arc;

use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_dynamodb::Client;

#[derive(Clone, Debug)]
pub struct SharedState {
    pub client: Arc<Client>,
    pub jwt_token: String,
}

impl SharedState {
    pub fn new(config: &SdkConfig, jwt_token: String) -> Self {
        let client = Arc::new(Client::new(&config));

        Self { client, jwt_token }
    }

    pub async fn get_config() -> SdkConfig {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        aws_config::from_env().region(region_provider).load().await
    }
}
