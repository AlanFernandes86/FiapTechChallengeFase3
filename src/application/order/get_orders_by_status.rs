use std::error::Error;
use crate::domain::{entities::order::Order, enums::order_status::EnOrderStatus, repository::{order_product_repository::OrderProductRepository, order_repository::OrderRepository}};

pub struct GetOrdersByStatusUseCase {
    order_repository: Box<dyn OrderRepository>
}

impl GetOrdersByStatusUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository
        }
    }

    pub async fn handle(&self, order_status_id: i32) -> Result<Option<Vec<Order>>, Box<dyn Error + Send + Sync>> {
        let mut status = vec![order_status_id];

        if order_status_id == EnOrderStatus::Active as i32 {
            status = vec![
                EnOrderStatus::Received as i32,
                EnOrderStatus::InPreparation as i32,
                EnOrderStatus::Ready as i32
            ];
        }

        let get_order_result = self.order_repository.get_orders_by_status(status).await;

        match get_order_result {
            Ok(Some(mut vec_order)) => {
                for order in vec_order.iter_mut() {
                    order.total = order.calculate_total();
                }
                Ok(Some(vec_order))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
    }
}