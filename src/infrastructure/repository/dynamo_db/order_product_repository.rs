use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use crate::domain::{entities::order_product::OrderProduct, repository::order_product_repository::OrderProductRepository};
use crate::infrastructure::repository::dynamo_db::common::dynamo_db_counters::DynamoDbCounters;

pub struct DynamoDbOrderProductRepository {
    client: Arc<Client>,
    counter: DynamoDbCounters
}

impl DynamoDbOrderProductRepository {
    pub fn new(client: Arc<Client>) -> Self {
        let counter = DynamoDbCounters::new(client.clone());
        DynamoDbOrderProductRepository { client: client, counter: counter }
    }
}

#[async_trait]
impl OrderProductRepository for DynamoDbOrderProductRepository {
    async fn put_order_product(&self, order_product: OrderProduct) -> Result<(), Box<dyn Error>> {
        let order_product_id = match order_product.order_product_id {
            Some(id) => id,
            None => self.counter.get_next_id("order_product").await?
        };
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        let mut order_product_item = std::collections::HashMap::new();
        order_product_item.insert("order_product_id".to_string(), AttributeValue::N(order_product_id.to_string()));
        order_product_item.insert("order_id".to_string(), AttributeValue::N(order_product.order_id.to_string()));
        order_product_item.insert("product_id".to_string(), AttributeValue::N(order_product.product_id.to_string()));
        order_product_item.insert("name".to_string(), AttributeValue::S(order_product.name.to_string()));
        order_product_item.insert("quantity".to_string(), AttributeValue::N(order_product.quantity.to_string()));
        order_product_item.insert("price".to_string(), AttributeValue::N(order_product.price.to_string()));
        order_product_item.insert("description".to_string(), AttributeValue::S(order_product.description.to_string()));
        order_product_item.insert("image_url".to_string(), AttributeValue::S(order_product.image_url.to_string()));
        order_product_item.insert("product_category_id".to_string(), AttributeValue::S(order_product.product_category.id.to_string()));
        order_product_item.insert("product_category_name".to_string(), AttributeValue::S(order_product.product_category.name.to_string()));
        order_product_item.insert("product_category_description".to_string(), AttributeValue::S(order_product.product_category.description.to_string()));
        order_product_item.insert("created_at".to_string(), AttributeValue::S(timestamp.clone()));
        order_product_item.insert("updated_at".to_string(), AttributeValue::S(timestamp.clone()));
        
        let result = self.client
            .update_item()
            .table_name("order")
            .key("pk", AttributeValue::S(format!("ORDER#{}", order_product.order_id)))
            .key("sk", AttributeValue::S("ORDER#details".to_string()))
            .update_expression("SET order_products.#id = :order_product")
            .expression_attribute_names("#id", order_product_id.to_string())
            .expression_attribute_values(":order_product", AttributeValue::M(order_product_item))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    async fn delete_order_product(&self, order_id: i32, order_product_id: i32) -> Result<(), Box<dyn Error>> {
        let result = self.client
            .update_item()
            .table_name("order")
            .key("pk", AttributeValue::S(format!("ORDER#{}", order_id)))
            .key("sk", AttributeValue::S("ORDER#details".to_string()))
            .update_expression("REMOVE order_products.#id")
            .expression_attribute_names("#id", order_product_id.to_string())
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }
}