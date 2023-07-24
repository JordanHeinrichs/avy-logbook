use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_dynamodb::Client;

#[derive(Clone, Debug)]
pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub fn new(config: &SdkConfig) -> Self {
        let client = Client::new(&config);

        Self { client: client }
    }

    pub async fn get_config() -> SdkConfig {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        aws_config::from_env().region(region_provider).load().await
    }
}
