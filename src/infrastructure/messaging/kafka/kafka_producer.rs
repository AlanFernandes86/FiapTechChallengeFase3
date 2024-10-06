use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use async_trait::async_trait;
use crate::domain::{entities::order::Order, messaging::event_publisher::EventPublisher};

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer })
    }
}

#[async_trait]
impl EventPublisher for KafkaProducer {
    async fn publish_order_status_update(&self, topic: &str, payload: &Order) -> Result<(), Box<dyn std::error::Error>> {
        let json_payload = serde_json::to_string(payload).unwrap();
        let key = payload.id.to_string();
        let record = FutureRecord::to(topic)
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
