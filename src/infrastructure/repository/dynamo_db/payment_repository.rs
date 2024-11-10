use std::{error::Error, sync::Arc};
use async_trait::async_trait;
use crate::domain::{entities::payment::Payment, repository::payment_repository::PaymentRepository};
use super::common::dynamo_db_counters::DynamoDbCounters;
use aws_sdk_dynamodb::{types::AttributeValue, Client};

pub struct DynamoDbPaymentRepository {
    client: Arc<Client>,
    counter: DynamoDbCounters
}

impl DynamoDbPaymentRepository {
    pub fn new(client: Arc<Client>) -> Self {
        let counter = DynamoDbCounters::new(client.clone());
        DynamoDbPaymentRepository { client: client, counter: counter }
    }
}

#[async_trait]
impl PaymentRepository for DynamoDbPaymentRepository {
    async fn put_payment(&self, payment: Payment) -> Result<(), Box<dyn Error>> {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let payment_id = self.counter.get_next_id("payment").await?;
        let mut payment_item = std::collections::HashMap::new();
        payment_item.insert("pk".to_string(), AttributeValue::S(format!("ORDER#{}", payment.order_id)));
        payment_item.insert("sk".to_string(), AttributeValue::S(format!("PAYMENT#{}", payment_id)));
        payment_item.insert("order_id".to_string(), AttributeValue::N(payment.order_id.to_string()));
        payment_item.insert("payment_status_id".to_string(), AttributeValue::N(payment.payment_status_id.to_string()));
        payment_item.insert("payment_method_id".to_string(), AttributeValue::N(payment.payment_method_id.to_string()));
        payment_item.insert("payment_method_order_id".to_string(), AttributeValue::N(payment.payment_method_order_id.to_string()));
        payment_item.insert("value".to_string(), AttributeValue::N(payment.value.to_string()));
        payment_item.insert("message".to_string(), AttributeValue::S(payment.message));
        payment_item.insert("origin".to_string(), AttributeValue::S(payment.origin));
        payment_item.insert("updated_at".to_string(), AttributeValue::S(timestamp.clone()));
        payment_item.insert("created_at".to_string(), AttributeValue::S(timestamp.clone()));
                        
        let result = self.client
            .put_item()
            .table_name("payment")
            .set_item(Some(payment_item))
            .send()
            .await;

        match result {
            Ok(_) => {
                let update_order_result = self.client
                    .update_item()
                    .table_name("order")
                    .key("pk", AttributeValue::S(format!("ORDER#{}", payment.order_id)))
                    .key("sk", AttributeValue::S("ORDER#details".to_string()))
                    .update_expression("SET order_payment_status_id = :payment_status_id")
                    .expression_attribute_values(":payment_status_id", AttributeValue::N(payment.payment_status_id.to_string()))
                    .send()
                    .await;

                match update_order_result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}