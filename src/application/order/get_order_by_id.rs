use std::error::Error;
use crate::domain::{entities::order::Order, repository::{order_product_repository::OrderProductRepository, order_repository::OrderRepository}};

pub struct GetOrderByIdUseCase {
    order_repository: Box<dyn OrderRepository>,
    order_product_repository: Box<dyn OrderProductRepository>
}

impl GetOrderByIdUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>, order_product_repository: Box<dyn OrderProductRepository>) -> Self {
        Self {
            order_repository,
            order_product_repository
        }
    }

    pub async fn handle(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error>> {
        let get_order_result = self.order_repository.get_order_by_id(order_id).await;
        match get_order_result {
            Ok(Some(mut order)) => {
                let order_products = self.order_product_repository.get_order_products(order_id).await?;
                order.order_products = order_products;
                order.total = order.calculate_total();
                Ok(Some(order))       
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
    }
}