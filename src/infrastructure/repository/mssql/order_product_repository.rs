use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MssqlPool;
use crate::domain::{entities::order_product::OrderProduct, repository::order_product_repository::OrderProductRepository};

pub struct MssqlOrderProductRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlOrderProductRepository {
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlOrderProductRepository { pool }
    }
}

#[async_trait]
impl OrderProductRepository for MssqlOrderProductRepository {
    async fn put_order_product(&self, order_product: OrderProduct) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            MERGE INTO TechChallenge.dbo.order_product AS target
            USING (VALUES (@p1, @p2, @p3, @p4, @p5, GETDATE())) AS source (id, order_id, product_id, quantity, price, updated_at)
            ON target.id = source.id
            WHEN MATCHED THEN
                UPDATE SET
                    quantity = source.quantity,
                    price = source.price,
                    updated_at = GETDATE()
            WHEN NOT MATCHED THEN
                INSERT (order_id, product_id, quantity, price, updated_at, created_at)
                VALUES (source.order_id, source.product_id, source.quantity, source.price, GETDATE(), GETDATE());
            "#
        )
        .bind(order_product.order_product_id)
        .bind(order_product.order_id)
        .bind(order_product.product_id)
        .bind(order_product.quantity)
        .bind(order_product.price)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    async fn delete_order_product(&self, order_id: i32, order_product_id: i32) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            DELETE FROM TechChallenge.dbo.order_product
            WHERE id = @p1 AND order_id = @p2;
            "#
        )
        .bind(order_product_id)
        .bind(order_id)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }
}