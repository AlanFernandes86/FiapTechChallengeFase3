use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::Client;
use crate::domain::entities::product_category::ProductCategory;
use crate::domain::repository::product_category_repository::ProductCategoryRepository;
use crate::infrastructure::repository::dynamo_db::entity::db_product_category::DbProductCategory;

pub struct DynamoDbProductCategoryRepository {
    client: Arc<Client>,
}

impl DynamoDbProductCategoryRepository {
    pub fn new(client: Arc<Client>) -> Self {
        DynamoDbProductCategoryRepository { client: client }
    }
}

#[async_trait]
impl ProductCategoryRepository for DynamoDbProductCategoryRepository {
    async fn get_product_categories(&self) -> Result<Option<Vec<ProductCategory>>, Box<dyn Error>> {
        let result = self.client
        .execute_statement()
        .statement("SELECT * FROM \"product\" WHERE begins_with(pk, 'CATEGORY#') AND sk = 'CATEGORY#metadata';")
        .send()
        .await;
        
        match result {
            Ok(output) => {
                let items = output.items.as_ref().map_or_else(|| Vec::new(), |items| items.clone());
                if items.is_empty() {
                    Ok(None)
                } else {
                    let dynamo_product_category: Vec<DbProductCategory> = items.into_iter().map(DbProductCategory::from_item).collect();
                    let domain_product_category: Vec<ProductCategory> = dynamo_product_category.into_iter().map(DbProductCategory::into_domain).collect();
                    Ok(Some(domain_product_category))
                }
            },
            Err(e) => {
                let error_message = format!(
                                    "Error querying DynamoDB: {:?}",
                                    e.as_service_error()
                                );
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
            }
        }
    }

    async fn put_product_category(&self, product_category: ProductCategory) -> Result<(), Box<dyn Error>> {
        let mut item = std::collections::HashMap::new();
        item.insert("pk".to_string(), AttributeValue::S(format!("CATEGORY#{}", product_category.id).to_string()));
        item.insert("sk".to_string(), AttributeValue::S("CATEGORY#metadata".to_string()));
        item.insert("id".to_string(), AttributeValue::N(product_category.id.to_string()));
        item.insert("name".to_string(), AttributeValue::S(product_category.name.to_string()));
        item.insert("description".to_string(), AttributeValue::S(product_category.description.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(chrono::Utc::now().to_rfc3339()));
        item.insert("created_at".to_string(), AttributeValue::S(chrono::Utc::now().to_rfc3339()));

        let result = self.client
            .put_item()
            .table_name("product")
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(pk) and attribute_not_exists(sk)")
            .send()
            .await;            

        match result {
            Ok(_) => {
                println!("Item inserted successfully.");
                Ok(())
            },
            Err(SdkError::ServiceError(err)) if err.err().code() == Some("ConditionalCheckFailedException") => {
                self.client
                    .update_item()
                    .table_name("product")
                    .key("pk", AttributeValue::S(format!("CATEGORY#{}", product_category.id)))
                    .key("sk", AttributeValue::S("CATEGORY#metadata".to_string()))
                    .update_expression("SET #name = :name, #description = :description, #updated_at = :updated_at")
                    .expression_attribute_names("#name", "name")
                    .expression_attribute_names("#description", "description")
                    .expression_attribute_names("#updated_at", "updated_at")
                    .expression_attribute_values(":name", AttributeValue::S(product_category.name.to_string()))
                    .expression_attribute_values(":description", AttributeValue::S(product_category.description.to_string()))
                    .expression_attribute_values(":updated_at", AttributeValue::S(chrono::Utc::now().to_rfc3339()))
                    .send()
                    .await?;
        
                println!("Item updated successfully.");
                Ok(())
            },
            Err(SdkError::ServiceError(err)) => {
                let error_message = format!("Service error querying DynamoDB: {:?}", err);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
            },
            Err(e) => {
                let error_message = format!("Unexpected error querying DynamoDB: {:?}", e);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
            }
        }
    }

    async fn delete_product_category(&self, id: i32) -> Result<u64, Box<dyn Error>> {
        let result = self.client
        .delete_item()
        .table_name("product")
        .key("pk", AttributeValue::S(format!("CATEGORY#{}", id).to_string()))
        .key("sk", AttributeValue::S("CATEGORY#metadata".to_string()))
        .return_values("ALL_OLD".into())
        .send()
        .await;

        match result {
            Ok(output) => {
                if output.attributes.is_some() {
                    println!("Item deleted successfully.");
                    Ok(1)
                } else {
                    println!("Item did not exist, nothing to delete.");
                    Ok(0)
                }            
            },
            Err(e) => {
                let error_message = format!(
                                    "Error querying DynamoDB: {:?}",
                                    e.as_service_error()
                                );
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
            }
        }
    }
}