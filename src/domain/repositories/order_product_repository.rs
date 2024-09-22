use std::error::Error;
use async_trait::async_trait;

use crate::domain::entities::order_product::OrderProduct;

#[async_trait]
pub trait OrderProductRepository {
	async fn put_order_product(&self, order_product: OrderProduct) -> Result<(), Box<dyn Error>>;
    async fn delete_order_product(&self, order_product_id: i32) -> Result<(), Box<dyn Error>>;
    async fn get_order_products(&self, order_id: i32) -> Result<Vec<OrderProduct>, Box<dyn Error>>;
}