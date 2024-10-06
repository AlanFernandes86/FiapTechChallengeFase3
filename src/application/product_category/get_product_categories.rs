use std::error::Error;
use crate::domain::{entities::product_category::ProductCategory, repository::product_category_repository::ProductCategoryRepository};

pub struct GetProductCategoriesUseCase {
    product_category_repository: Box<dyn ProductCategoryRepository>,
}

impl GetProductCategoriesUseCase {
    pub fn new(product_category_repository: Box<dyn ProductCategoryRepository>) -> Self {
        GetProductCategoriesUseCase {
            product_category_repository
        }
    }

    pub async fn handle(&self) -> Result<Option<Vec<ProductCategory>>, Box<dyn Error>> {
        self.product_category_repository.get_product_categories().await
    }
}