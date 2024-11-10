use std::error::Error;
use async_trait::async_trait;
use crate::domain::entities::product::Product;

#[async_trait]
pub trait ProductRepository {
    async fn get_product_by_id(&self, product_id: i32) -> Result<Option<Product>, Box<dyn Error>>;
    async fn get_products_by_category(&self, product_category_id: i32) -> Result<Option<Vec<Product>>, Box<dyn Error>>;
    async fn put_product(&self, product: Product) -> Result<(), Box<dyn Error>>;
}