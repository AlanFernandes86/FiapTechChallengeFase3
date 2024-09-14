use std::error::Error;
use crate::domain::{entities::order::Order, enums::order_status::OrderStatus, repositories::order_repository::OrderRepository};

pub struct GetOrdersByStatusUseCase {
    order_repository: Box<dyn OrderRepository>,
}

impl GetOrdersByStatusUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository,
        }
    }

    pub async fn handle(&self, order_status_id: i32) -> Result<Option<Vec<Order>>, Box<dyn Error>> {
        let mut status = vec![order_status_id];

        if order_status_id == OrderStatus::Active as i32 {
            status = vec![
                OrderStatus::Received as i32,
                OrderStatus::InPreparation as i32,
                OrderStatus::Ready as i32
            ];
        }

        self.order_repository.get_orders_by_status(status).await
    }
}