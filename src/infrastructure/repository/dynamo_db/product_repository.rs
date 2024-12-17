use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::Client;
use crate::domain::entities::product::Product;
use crate::domain::entities::product_category::ProductCategory;
use crate::domain::repository::product_repository::ProductRepository;
use crate::infrastructure::repository::dynamo_db::entity::db_product::DbProduct;
use crate::infrastructure::repository::dynamo_db::entity::db_product_category::DbProductCategory;
use crate::infrastructure::repository::dynamo_db::common::dynamo_db_counters::DynamoDbCounters;

pub struct DynamoDbProductRepository {
    client: Arc<Client>,
    counter: DynamoDbCounters
}

impl DynamoDbProductRepository {
    pub fn new(client: Arc<Client>) -> Self {
        let counter = DynamoDbCounters::new(client.clone());
        DynamoDbProductRepository { client: client, counter: counter }
    }
}

#[async_trait]
impl ProductRepository for DynamoDbProductRepository {
    async fn get_products(&self) -> Result<Option<HashMap<i32, Product>>, Box<dyn Error>> {
        let result = self.client
            .scan()
            .table_name("product")
            .send()
            .await;
        
        match result {
            Ok(output) => {
                if output.items.is_none() {
                    return Ok(None);
                }
                if let Some(items) = output.items {
                    let mut products: Vec<(i32, Product)> = vec![];
                    let mut product_categories: HashMap<i32, ProductCategory> = HashMap::new();

                    for item in items {
                        if let Some(sk) = item.get("sk") {
                            match sk.as_s().map(|s| s.as_str()) {
                                Ok("CATEGORY#metadata") => {
                                    println!("Product category found");
                                    let product_category_db = DbProductCategory::from_item(item);
                                    let product_category = ProductCategory {
                                        id: product_category_db.id,
                                        name: product_category_db.name,
                                        description: product_category_db.description,
                                    };
                                    product_categories.insert(product_category_db.id, product_category);
                                }
                                Ok(_) => {
                                    println!("Product found");
                                    let category_id = item.get("pk")
                                        .and_then(|v| v.as_s().ok())
                                        .and_then(|v| v.split("#").last())
                                        .and_then(|v| v.parse::<i32>().ok())
                                        .unwrap_or_default();
                                    let product = DbProduct::from_item(item).into_domain();
                                    products.push((category_id, product));
                                },
                                Err(_) => {
                                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error parsing product category")));
                                }
                            }
                        }
                    }

                    for product in products.iter_mut() {
                        if let Some(product_category) = product_categories.get(&product.0) {
                            product.1.product_category = product_category.clone();
                        }
                    }

                    let products_with_id: HashMap<i32, Product> = products.into_iter().map(|(_, v)| (v.id.unwrap(), v)).collect();

                     Ok(Some(products_with_id))
                } else {
                    Ok(None)
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

    async fn get_products_by_category(&self, product_category_id: i32) -> Result<Option<Vec<Product>>, Box<dyn Error>> {
      let result = self.client
      .query()
      .table_name("product")
      .key_condition_expression("pk = :product_category_id")
      .expression_attribute_values(":product_category_id", AttributeValue::S(format!("CATEGORY#{}", product_category_id).to_string()))
      .send()
      .await;

      match result {
        Ok(output) => {
           if output.items.is_none() {
              return Ok(None);
           } 
           if let Some(items) = output.items {
            let mut products: Vec<Product> = Vec::new();
            let mut product_category: ProductCategory = ProductCategory::default();

            for item in items {
              if let Some(sk) = item.get("sk") {
                match sk.as_s().map(|s| s.as_str()) {
                  Ok("CATEGORY#metadata") => {
                    println!("Product category found");
                    let product_category_db = DbProductCategory::from_item(item);
                    product_category.id = product_category_db.id;
                    product_category.name = product_category_db.name;
                    product_category.description = product_category_db.description;
                  }
                  Ok(_) => {
                    println!("Product found");
                    let product = DbProduct::from_item(item).into_domain();
                    products.push(product);
                  },
                  Err(_) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error parsing product category")));
                  }
                }
              }
            }

            for product in products.iter_mut() {
              println!("Setting product category for product");
              product.product_category = product_category.clone();
            }

            Ok(Some(products))
          }
          else {
            Ok(None)
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

    async fn put_product(&self, product: Product) -> Result<(), Box<dyn Error>> {
      let id = match product.id {
          Some(id) => id,
          None => self.counter.get_next_id("product").await?,
      };
      let mut item = std::collections::HashMap::new();
      item.insert("pk".to_string(), AttributeValue::S(format!("CATEGORY#{}", product.product_category.id).to_string()));
      item.insert("sk".to_string(), AttributeValue::S(format!("PRODUCT#{}", id).to_string()));
      item.insert("id".to_string(), AttributeValue::N(id.to_string()));
      item.insert("name".to_string(), AttributeValue::S(product.name.to_string()));
      item.insert("description".to_string(), AttributeValue::S(product.description.to_string()));
      item.insert("price".to_string(), AttributeValue::N(product.price.to_string()));
      item.insert("image_url".to_string(), AttributeValue::S(product.image_url.to_string()));
      item.insert("available".to_string(), AttributeValue::Bool(product.available));
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
                  .key("pk", AttributeValue::S(format!("CATEGORY#{}", product.product_category.id)))
                  .key("sk", AttributeValue::S(format!("PRODUCT#{}", id)))
                  .update_expression("SET #name = :name, #description = :description, #price = :price, #image_url = :image_url, #available = :available, #updated_at = :updated_at")
                  .expression_attribute_names("#name", "name")
                  .expression_attribute_names("#description", "description")
                  .expression_attribute_names("#price", "price")
                  .expression_attribute_names("#image_url", "image_url")
                  .expression_attribute_names("#available", "available")
                  .expression_attribute_names("#updated_at", "updated_at")
                  .expression_attribute_values(":name", AttributeValue::S(product.name.to_string()))
                  .expression_attribute_values(":description", AttributeValue::S(product.description.to_string()))
                  .expression_attribute_values(":price", AttributeValue::N(product.price.to_string()))
                  .expression_attribute_values(":image_url", AttributeValue::S(product.image_url.to_string()))
                  .expression_attribute_values(":available", AttributeValue::Bool(product.available))
                  .expression_attribute_values(":updated_at", AttributeValue::S(chrono::Utc::now().to_rfc3339()))
                  .send()
                  .await?;
          
              println!("Item updated successfully.");
              Ok(())
          },
          Err(e) => {
              let error_message = format!("Unexpected error querying DynamoDB: {:?}", e);
              Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
          }
      }

    }
}