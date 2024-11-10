use std::error::Error;
use crate::domain::{entities::order::Order, repository::order_repository::OrderRepository};

pub struct GetOrderByIdUseCase {
    order_repository: Box<dyn OrderRepository>
}

impl GetOrderByIdUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository
        }
    }

    pub async fn handle(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error + Send + Sync>> {
        let get_order_result = self.order_repository.get_order_by_id(order_id).await;
        match get_order_result {
            Ok(Some(mut order)) => {
                order.total = order.calculate_total();
                Ok(Some(order))       
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
    }
}