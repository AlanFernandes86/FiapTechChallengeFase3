use crate::domain::entities::order::Order;
use async_trait::async_trait;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish_order_status_update(&self, payload: &Order) -> Result<(), Box<dyn std::error::Error>>;
}