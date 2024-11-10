use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::vec;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes, PutRequest, WriteRequest};
use aws_sdk_dynamodb::Client;
use futures::future::join_all;
use crate::domain::entities::order::{Order, OrderPayment, OrderStatus};
use crate::domain::entities::order_product::OrderProduct;
use crate::domain::enums::order_status::EnOrderStatus;
use crate::domain::repository::order_repository::OrderRepository;
use crate::infrastructure::repository::dynamo_db::common::dynamo_db_counters::DynamoDbCounters;
use crate::infrastructure::repository::dynamo_db::entity::db_order::DbOrder;
use crate::infrastructure::repository::dynamo_db::entity::db_order_product::DbOrderProduct;

pub struct DynamoDbOrderRepository {
    client: Arc<Client>,
    counter: DynamoDbCounters
}

impl DynamoDbOrderRepository {
    pub fn new(client: Arc<Client>) -> Self {
        let counter = DynamoDbCounters::new(client.clone());
        DynamoDbOrderRepository { client: client, counter: counter }
    }
}

#[async_trait]
impl OrderRepository for DynamoDbOrderRepository {
    async fn get_order_by_id(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error + Send + Sync>> {
        let result = self.client
            .get_item()
            .table_name("order")
            .key("pk", AttributeValue::S(format!("ORDER#{}", order_id)))
            .key("sk", AttributeValue::S("ORDER#details".to_string()))
            .send()
            .await;

        match result {
            Ok(output) => {
                if let Some(item) = output.item {
                    let db_order = DbOrder::from_item(item);
                    Ok(Some(db_order.into()))
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
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)));
            }
        }        
    }

    async fn get_orders_by_status(&self, order_status_list: Vec<i32>) -> Result<Option<Vec<Order>>, Box<dyn Error + Send + Sync>> {
        let futures = order_status_list.iter().map(|&status| {
            self.client
                .query()
                .table_name("order")
                .index_name("order_status_id_index")
                .key_condition_expression("#id = :id_value")
                .expression_attribute_names("#id", "order_status_id")
                .expression_attribute_values(":id_value", AttributeValue::N(status.to_string()))
                .send()
        });

        let results = join_all(futures).await;
       
        let mut orders: Vec<Order> = Vec::new();
        for result in results {
            match result {
                Ok(output) => {
                    if let Some(items) = output.items {
                        for item in items {
                            let db_order = DbOrder::from_item(item);
                            orders.push(db_order.into());
                        }
                    }
                },
                Err(e) => {
                    let error_message = format!(
                        "Error querying DynamoDB: {:?}",
                        e.as_service_error()
                    );
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)));
                }
            }
        }

        Ok(Some(orders))       
    }

    async fn create_order(&self, order: Order) -> Result<i32, Box<dyn Error>> {
        let order_id = self.counter.get_next_id("order").await?;
        let timestamp = chrono::Utc::now().to_rfc3339();
    
        let mut order_item = std::collections::HashMap::new();
        order_item.insert("pk".to_string(), AttributeValue::S(format!("ORDER#{}", order_id)));
        order_item.insert("sk".to_string(), AttributeValue::S("ORDER#details".to_string()));
        order_item.insert("order_id".to_string(), AttributeValue::N(order_id.to_string()));
        order_item.insert("client_name".to_string(), AttributeValue::S(order.client.name.to_string()));
        order_item.insert("client_cpf".to_string(), AttributeValue::S(order.client.cpf.to_string()));
        order_item.insert("order_status_id".to_string(), AttributeValue::N(order.order_status.id.to_string()));
        order_item.insert("created_at".to_string(), AttributeValue::S(timestamp.clone()));
        order_item.insert("updated_at".to_string(), AttributeValue::S(timestamp.clone()));

        let mut order_products = HashMap::new();
        for order_product in &order.order_products {
            let order_product_id = self.counter.get_next_id("order_product").await?;
            let mut order_product_item = std::collections::HashMap::new();
            order_product_item.insert("order_product_id".to_string(), AttributeValue::N(order_product_id.to_string()));
            order_product_item.insert("order_id".to_string(), AttributeValue::N(order_id.to_string()));
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

            order_products.insert(order_product_id.to_string(), AttributeValue::M(order_product_item));
        }

        order_item.insert("order_products".to_string(), AttributeValue::M(order_products));
    
        let result = self.client
            .put_item()
            .table_name("order")
            .set_item(Some(order_item))
            .send()
            .await;

        match result {
            Ok(_) => {
                println!("Order id [{}] created successfully.", order_id);
                Ok(order_id)
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

    async fn update_order_status(&self, order_id: i32, order_status_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        let result = self.client
            .update_item()
            .table_name("order")
            .key("pk", AttributeValue::S(format!("ORDER#{}", order_id)))
            .key("sk", AttributeValue::S("ORDER#details".to_string()))
            .update_expression("SET order_status_id = :order_status_id")
            .expression_attribute_values(":order_status_id", AttributeValue::N(order_status_id.to_string()))
            .send()
            .await;

        match result {
            Ok(_) => {
                println!("Order id [{}] status updated successfully.", order_id);
                Ok(())
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
