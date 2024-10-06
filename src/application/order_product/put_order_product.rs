use std::error::Error;
use crate::domain::{entities::order_product::OrderProduct, repository::order_product_repository::OrderProductRepository};

pub struct PutOrderProductUseCase {
    order_product_repository: Box<dyn OrderProductRepository>
}

impl PutOrderProductUseCase {
    pub fn new(order_product_repository: Box<dyn OrderProductRepository>) -> Self {
        Self {
            order_product_repository
        }
    }

    pub async fn handle(&self, order_product: OrderProduct) -> Result<(), Box<dyn Error>> {
        self.order_product_repository.put_order_product(order_product).await
    }
}

