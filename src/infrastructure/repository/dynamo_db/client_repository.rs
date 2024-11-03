use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::Client as DynamoClient;
use crate::domain::errors::client_already_exists_error::ClientAlreadyExistsError;
use crate::domain::repository::client_repository::ClientRepository;
use crate::domain::entities::client::Client;
use crate::infrastructure::repository::dynamo_db::entity::db_client::DbClient;

pub struct DynamoDbClientRepository {
  client: Arc<DynamoClient>,
}

impl DynamoDbClientRepository {
  pub fn new(client: Arc<DynamoClient>) -> Self {
      DynamoDbClientRepository { client: client }
  }
}

#[async_trait]
impl ClientRepository for DynamoDbClientRepository {
    async fn get_client_by_cpf(&self, cpf: String) -> Result<Option<Client>, Box<dyn Error>> {
        let result = self.client
        .get_item()
        .table_name("client")
        .key("cpf", AttributeValue::S(cpf))
        .send()
        .await;

        match result {
            Ok(output) => {
                if output.item.is_none() {
                  return Ok(None);
                }
                let item = output.item.unwrap();
                let client = DbClient::from_item(item).into();
                Ok(Some(client))
            },
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }

    async fn set_client(&self, client: Client) -> Result<(), Box<dyn Error>> {
      let mut item = std::collections::HashMap::new();
      item.insert("cpf".to_string(), AttributeValue::S(client.cpf.clone()));
      item.insert("name".to_string(), AttributeValue::S(client.name));
      item.insert("email".to_string(), AttributeValue::S(client.email));
      item.insert("updated_at".to_string(), AttributeValue::S(chrono::Utc::now().to_rfc3339()));
      item.insert("created_at".to_string(), AttributeValue::S(chrono::Utc::now().to_rfc3339()));
      
      let result = self.client
      .put_item()
      .table_name("client")
      .set_item(Some(item))
      .condition_expression("attribute_not_exists(cpf)")
      .send()
      .await;

      match result {
          Ok(_) => Ok(()),
          Err(SdkError::ServiceError(err)) if err.err().code() == Some("ConditionalCheckFailedException") => {
              Err(Box::new(ClientAlreadyExistsError::new(client.cpf)))
          },
          Err(e) => {
              let error_message = format!("Error querying DynamoDB: {:?}", e);
              Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
          }
      }
    }
}