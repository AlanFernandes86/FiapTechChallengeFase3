use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::Client as DynamoClient;
use crate::domain::errors::user_already_exists_error::UserAlreadyExistsError;
use crate::domain::repository::user_repository::UserRepository;
use crate::domain::entities::user::User;
use crate::infrastructure::repository::dynamo_db::entity::db_user::DbUser;

pub struct DynamoDbUserRepository {
  client: Arc<DynamoClient>,
}

impl DynamoDbUserRepository {
  pub fn new(client: Arc<DynamoClient>) -> Self {
      DynamoDbUserRepository { client: client }
  }
}

#[async_trait]
impl UserRepository for DynamoDbUserRepository {
    async fn get_user_by_cpf(&self, cpf: String) -> Result<Option<User>, Box<dyn Error>> {
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
                let client = DbUser::from_item(item).into();
                Ok(Some(client))
            },
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }

    async fn set_user(&self, user: User) -> Result<(), Box<dyn Error>> {
      let mut item = std::collections::HashMap::new();
      item.insert("cpf".to_string(), AttributeValue::S(user.cpf.clone()));
      item.insert("name".to_string(), AttributeValue::S(user.name));
      item.insert("email".to_string(), AttributeValue::S(user.email));
      item.insert("group".to_string(), AttributeValue::S(user.group));
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
              Err(Box::new(UserAlreadyExistsError::new(user.cpf)))
          },
          Err(e) => {
              let error_message = format!("Error querying DynamoDB: {:?}", e);
              Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
          }
      }
    }
}