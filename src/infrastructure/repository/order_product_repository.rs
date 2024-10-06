use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MssqlPool;
use crate::domain::{entities::order_product::OrderProduct, repository::order_product_repository::OrderProductRepository};

use super::entity::db_order_product::DbOrderProduct;

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

    async fn delete_order_product(&self, order_product_id: i32) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            DELETE FROM TechChallenge.dbo.order_product
            WHERE id = @p1
            "#
        )
        .bind(order_product_id)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    async fn get_order_products(&self, order_id: i32) -> Result<Vec<OrderProduct>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbOrderProduct>(
            r#"
            SELECT
                op.id AS order_product_id,
                p.id AS product_id,
                p.name,
                op.order_id,
                op.quantity,
                op.price,
                p.description,
                p.image_url,
                pc.id AS product_category_id,
                pc.name AS product_category_name,
                pc.description AS product_category_description,
                op.updated_at,
                op.created_at
            FROM
                TechChallenge.dbo.order_product op
                JOIN TechChallenge.dbo.product p ON op.product_id = p.id
                JOIN TechChallenge.dbo.product_category pc ON p.product_category_id = pc.id
            WHERE
                op.order_id = @p1
            "#
        )
        .bind(order_id)
        .fetch_all(&*self.pool)
        .await;

        match result {
            Ok(vec) => {
                if vec.is_empty() {
                    Ok(vec![])
                } else {
                    let order_products: Vec<OrderProduct> = vec.into_iter().map(Into::into).collect();
                    Ok(order_products)
                }
            },
            Err(e) => Err(Box::new(e))
        }
    }
}