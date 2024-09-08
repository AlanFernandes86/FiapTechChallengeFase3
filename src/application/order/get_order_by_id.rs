use std::error::Error;
use crate::domain::{entities::order::Order, repositories::order_repository::OrderRepository};

pub struct GetOrderByIdUseCase {
    order_repository: Box<dyn OrderRepository>,
}

impl GetOrderByIdUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository,
        }
    }

    pub async fn handle(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error>> {
        self.order_repository.get_order_by_id(order_id).await
    }
}