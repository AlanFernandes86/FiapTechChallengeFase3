use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::product::Product;
use crate::domain::repository::product_repository::ProductRepository;
use crate::infrastructure::repository::mssql::entity::db_product::DbProduct;

pub struct MssqlProductRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlProductRepository {
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlProductRepository { pool }
    }
}

#[async_trait]
impl ProductRepository for MssqlProductRepository {
    async fn get_products_by_category(&self, product_category_id: i32) -> Result<Option<Vec<Product>>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbProduct>(
            "SELECT p.id, p.name, p.description, p.price, p.image_url, p.product_category_id, pc.name as product_category_name, pc.description as product_category_description
            FROM TechChallenge.dbo.product p
            JOIN TechChallenge.dbo.product_category pc ON p.product_category_id = pc.id
            WHERE p.product_category_id = @p1"
        )
        .bind(product_category_id)
        .fetch_all(&*self.pool)
        .await;

        match result {
            Ok(vec) => {
                if vec.is_empty() {
                    Ok(None)
                } else {
                    let product_vec: Vec<Product> = vec.into_iter().map(Into::into).collect();
                    Ok(Some(product_vec))
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn put_product(&self, product: Product) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            "MERGE INTO TechChallenge.dbo.product AS target
            USING (VALUES (@p1, @p2, @p3, @p4, @p5, @p6, GETDATE(), GETDATE())) AS source (id, name, description, price, product_category_id, image_url, updated_at, created_at)
            ON target.id = source.id
            WHEN MATCHED THEN
                UPDATE SET
                    target.name = source.name,
                    target.description = source.description,
                    target.price = source.price,
                    target.product_category_id = source.product_category_id,
                    target.image_url = source.image_url,
                    target.updated_at = source.updated_at
            WHEN NOT MATCHED THEN
                INSERT (name, description, price, product_category_id, image_url, updated_at, created_at)
                VALUES (source.name, source.description, source.price, source.product_category_id, source.image_url, source.updated_at, source.created_at);"
        )
        .bind(product.id)
        .bind(product.name)
        .bind(product.description)
        .bind(product.price)
        .bind(product.product_category.id)
        .bind(product.image_url)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
