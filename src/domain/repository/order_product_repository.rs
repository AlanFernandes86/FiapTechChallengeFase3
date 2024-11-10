use std::error::Error;
use async_trait::async_trait;

use crate::domain::entities::order_product::OrderProduct;

#[async_trait]
pub trait OrderProductRepository: Send + Sync {
	async fn put_order_product(&self, order_product: OrderProduct) -> Result<(), Box<dyn Error>>;
    async fn delete_order_product(&self, order_id: i32, order_product_id: i32) -> Result<(), Box<dyn Error>>;
}