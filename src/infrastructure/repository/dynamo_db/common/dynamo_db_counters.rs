use std::sync::Arc;
use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};

pub struct DynamoDbCounters {
    client: Arc<Client>,
    table_name: String,
}

impl DynamoDbCounters {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client, table_name: "counters".to_string() }
    }

    pub async fn get_next_id(&self, counter_name: &str) -> Result<i32, Error> {
        let response = self.client
          .update_item()
          .table_name(&self.table_name)
          .key("CounterName", AttributeValue::S(counter_name.to_string()))
          .update_expression("ADD CurrentValue :inc")
          .expression_attribute_values(":inc", AttributeValue::N("1".to_string()))
          .return_values("UPDATED_NEW".into())
          .send()
          .await;

        match response {
            Ok(output) => {
                let item = output.attributes.unwrap();
                let current_value = item.get("CurrentValue").unwrap().as_n().unwrap();
                Ok(current_value.parse::<i32>().unwrap())
            },
            Err(e) => Err(e.into())
        }
    }
      
}
