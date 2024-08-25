use std::error::Error;
use async_trait::async_trait;
use crate::domain::entities::product_category::ProductCategory;

#[async_trait]
pub trait ProductCategoryRepository {
    async fn get_product_categories(&self) -> Result<Option<Vec<ProductCategory>>, Box<dyn Error>>;
    async fn put_product_category(&self, product_category: ProductCategory) -> Result<(), Box<dyn Error>>;
    async fn delete_product_category(&self, id: i32) -> Result<(), Box<dyn Error>>;
}