use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use async_trait::async_trait;
use crate::domain::{entities::order::Order, messaging::event_publisher::EventPublisher};

pub struct KafkaProducer {
    producer: FutureProducer,
    order_status_update_topic: String
}

impl KafkaProducer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bootstraps = std::env::var("KAFKA_BOOTSTRAP_SERVERS").expect("KAFKA_BOOTSTRAP_SERVERS not found in .env file");
        let order_status_update_topic = std::env::var("KAFKA_ORDER_STATUS_UPDATE_TOPIC").expect("KAFKA_ORDER_STATUS_UPDATE_TOPIC not found in .env file");
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", bootstraps)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer, order_status_update_topic })
    }
}

#[async_trait]
impl EventPublisher for KafkaProducer {
    async fn publish_order_status_update(&self, payload: &Order) -> Result<(), Box<dyn std::error::Error>> {
        let json_payload = serde_json::to_string(payload).unwrap();
        let key = payload.id.to_string();
        let record = FutureRecord::to(&self.order_status_update_topic)
            .payload(&json_payload)
            .key(&key);

        let result = self.producer.send(record, std::time::Duration::from_secs(0)).await;
        
        match result {
            Ok(delivery) => {
                println!("Mensagem entregue em offset {:?}", delivery);
                Ok(())
            },
            Err((e, _)) => Err(Box::new(e))
        }
    }
}
