use std::error::Error;
use crate::domain::{entities::product::Product, repositories::product_repository::ProductRepository};

pub struct GetProductsByCategoryUseCase {
    product_repository: Box<dyn ProductRepository>,
}

impl GetProductsByCategoryUseCase {
    pub fn new(product_repository: Box<dyn ProductRepository>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn handle(&self, product_category_id: i32) -> Result<Option<Vec<Product>>, Box<dyn Error>> {
        self.product_repository.get_products_by_category(product_category_id).await
    }
}