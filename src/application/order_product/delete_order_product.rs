use std::error::Error;
use crate::domain::repository::order_product_repository::OrderProductRepository;

pub struct DeleteOrderProductUseCase {
    order_product_repository: Box<dyn OrderProductRepository>
}

impl DeleteOrderProductUseCase {
    pub fn new(order_product_repository: Box<dyn OrderProductRepository>) -> Self {
        Self {
            order_product_repository
        }
    }

    pub async fn handle(&self, order_id: i32, order_product_id: i32) -> Result<(), Box<dyn Error>> {
        self.order_product_repository.delete_order_product(order_id, order_product_id).await
    }
}