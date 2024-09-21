use std::error::Error;
use crate::domain::{entities::order::Order, enums::order_status::OrderStatus, repositories::order_repository::OrderRepository};

pub struct CreateOrderUseCase {
    order_repository: Box<dyn OrderRepository>,
}

impl CreateOrderUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository,
        }
    }

    pub async fn handle(&self, order: Order) -> Result<i32, Box<dyn Error>> {
        if order.order_status.id != OrderStatus::Created as i32 {
            return Err("order_status_id invalid! It is only possible to create orders with the status CREATED.".into());
        }

        self.order_repository.create_order(order).await
    }
}