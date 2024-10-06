use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::product_category::ProductCategory;
use crate::domain::repository::product_category_repository::ProductCategoryRepository;
use crate::infrastructure::repository::entity::db_product_category::DbProductCategory;

pub struct MssqlProductCategoryRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlProductCategoryRepository {
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlProductCategoryRepository { pool }
    }
}

#[async_trait]
impl ProductCategoryRepository for MssqlProductCategoryRepository {
    async fn get_product_categories(&self) -> Result<Option<Vec<ProductCategory>>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbProductCategory>(
            "SELECT id, name, description FROM TechChallenge.dbo.product_category"
        )
        .fetch_all(&*self.pool)
        .await;
        
        match result {
            Ok(vec) => {
                if vec.is_empty() {
                    Ok(None)
                } else {
                    let product_category_vec: Vec<ProductCategory> = vec.into_iter().map(Into::into).collect();
                    Ok(Some(product_category_vec))
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn put_product_category(&self, product_category: ProductCategory) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            "MERGE INTO TechChallenge.dbo.product_category AS target
            USING (VALUES (@p1, @p2, @p3, GETDATE(), GETDATE())) AS source (id, name, description, updated_at, created_at)
            ON target.id = source.id
            WHEN MATCHED THEN
                UPDATE SET target.name = source.name, target.description = source.description, target.updated_at = source.updated_at
            WHEN NOT MATCHED THEN
                INSERT (id, name, description, updated_at, created_at) VALUES (source.id, source.name, source.description, source.updated_at, source.created_at);"
        )
        .bind(product_category.id)
        .bind(product_category.name)
        .bind(product_category.description)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_product_category(&self, id: i32) -> Result<u64, Box<dyn Error>> {
        let result = sqlx::query(
            "DELETE FROM TechChallenge.dbo.product_category WHERE id = @p1"
        )
        .bind(id)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(res) => Ok(res.rows_affected()),
            Err(e) => Err(Box::new(e)),
        }
    }
}