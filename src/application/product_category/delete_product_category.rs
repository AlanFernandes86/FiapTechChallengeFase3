use std::error::Error;

use crate::domain::repository::product_category_repository::ProductCategoryRepository;

pub struct DeleteProductCategoryUseCase {
    product_category_repository: Box<dyn ProductCategoryRepository>,
}

impl DeleteProductCategoryUseCase {
    pub fn new(product_category_repository: Box<dyn ProductCategoryRepository>) -> Self {
        DeleteProductCategoryUseCase {
            product_category_repository
        }
    }

    pub async fn handle(&self, id: i32) -> Result<u64, Box<dyn Error>> {
        self.product_category_repository.delete_product_category(id).await
    }
}