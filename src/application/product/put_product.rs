use std::error::Error;
use crate::domain::{entities::product::Product, repository::product_repository::ProductRepository};

pub struct PutProductUseCase {
    product_repository: Box<dyn ProductRepository>,
}

impl PutProductUseCase {
    pub fn new(product_repository: Box<dyn ProductRepository>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn handle(&self, product: Product) -> Result<(), Box<dyn Error>> {
        self.product_repository.put_product(product).await
    }
}