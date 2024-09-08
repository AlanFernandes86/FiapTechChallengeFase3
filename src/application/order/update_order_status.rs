use std::error::Error;
use crate::domain::repositories::order_repository::OrderRepository;

pub struct UpdateOrderStatusUseCase {
    order_repository: Box<dyn OrderRepository>,
}

impl UpdateOrderStatusUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository,
        }
    }

    pub async fn handle(&self, order_id: i32, order_status_id: i32) -> Result<(), Box<dyn Error>> {
        self.order_repository.update_order_status(order_id, order_status_id).await
    }
}