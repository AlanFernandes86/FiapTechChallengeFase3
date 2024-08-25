use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait OrderProductRepository {
	// async fn put_order_product(&self, order_product: OrderProduct) -> Result<(), Box<dyn Error>>;
    // async fn delete_order_product(&self, order_product_id: i32) -> Result<(), Box<dyn Error>>;
}