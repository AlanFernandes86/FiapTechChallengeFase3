use std::sync::Arc;
use once_cell::sync::OnceCell;
use aws_sdk_dynamodb::{Client,Error};
use aws_config;

pub struct DynamoDbFactory {
    client: Arc<Client>,
}

// Static instance for singleton
static INSTANCE: OnceCell<DynamoDbFactory> = OnceCell::new();

impl DynamoDbFactory {
    pub async fn get_instance() -> Result<Arc<Client>, Error> {
         if INSTANCE.get().is_none() {
            let config = aws_config::load_from_env().await;
            let client = Client::new(&config);
            let dynamo_db_client = DynamoDbFactory {
                client: Arc::new(client),
            };
            let _ = INSTANCE.set(dynamo_db_client);
        }

        Ok(INSTANCE.get().unwrap().client.clone())
    }
}