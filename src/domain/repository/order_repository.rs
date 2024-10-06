use std::error::Error;
use async_trait::async_trait;
use crate::domain::entities::order::Order;

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn get_order_by_id(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error + Send + Sync>>;
    async fn get_orders_by_status(&self, order_status_list: Vec<i32>) -> Result<Option<Vec<Order>>, Box<dyn Error>>;
    async fn create_order(&self, order: Order) -> Result<i32, Box<dyn Error>>;
    async fn update_order_status(&self, order_id: i32, order_status_id: i32) -> Result<(), Box<dyn Error + Send + Sync>>;
}