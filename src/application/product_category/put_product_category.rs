use std::error::Error;
use crate::domain::{entities::product_category::ProductCategory, repositories::product_category_repository::ProductCategoryRepository};

pub struct PutProductCategoryUseCase {
    product_category_repository: Box<dyn ProductCategoryRepository>,
}

impl PutProductCategoryUseCase {
    pub fn new(product_category_repository: Box<dyn ProductCategoryRepository>) -> Self {
        PutProductCategoryUseCase {
            product_category_repository
        }
    }

    pub async fn handle(&self, product_category: ProductCategory) -> Result<(), Box<dyn Error>> {
        self.product_category_repository.put_product_category(product_category).await
    }
}