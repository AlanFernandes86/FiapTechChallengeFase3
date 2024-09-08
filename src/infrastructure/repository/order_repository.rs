use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::order::{Order, OrderProduct};
use crate::domain::repositories::order_repository::OrderRepository;
use crate::infrastructure::repository::entity::db_order::{DbOrder, DbOrderProduct};

pub struct MssqlOrderRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlOrderRepository {
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlOrderRepository { pool }
    }

    pub async fn get_order_products(&self, order_id: i32) -> Result<Vec<OrderProduct>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbOrderProduct>(
            r#"
            SELECT
                op.id AS order_product_id,
                p.id AS product_id,
                p.name,
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
                order_products op
                JOIN products p ON op.product_id = p.id
                JOIN product_categories pc ON p.product_category_id = pc.id
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

#[async_trait]
impl OrderRepository for MssqlOrderRepository {
    async fn get_order_by_id(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error>> {
        let result_order = sqlx::query_as::<_, DbOrder>(
            r#"
            SELECT
                o.id,
                c.cpf AS client_cpf,
                c.name AS client_name,
                c.email AS client_email,
                os.id AS order_status_id,
                os.name AS order_status_name,
                o.updated_at,
                o.created_at
            FROM
                orders o
                JOIN client c ON o.client_id = c.id
                JOIN order_status os ON o.order_status_id = os.id
            WHERE
                o.id = @p1
            "#
        )
        .bind(order_id)
        .fetch_optional(&*self.pool)
        .await;

        match result_order {
            Ok(Some(db_order)) => {
                let order_products = self.get_order_products(order_id).await?;
                let mut domain_order: Order = db_order.into();
                domain_order.order_products = order_products;        
                Ok(Some(domain_order))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(Box::new(e))
        }
    }

    async fn get_orders_by_status(&self, order_status_list: Vec<i32>) -> Result<Option<Vec<Order>>, Box<dyn Error>> {
        let placeholders = order_status_list
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");

        // Criar a query SQL dinamicamente.
        let query = format!(
            r#"
            SELECT
                o.id,
                c.cpf AS client_cpf,
                c.name AS client_name,
                c.email AS client_email,
                os.id AS order_status_id,
                os.name AS order_status_name,
                o.updated_at,
                o.created_at
            FROM
                orders o
                JOIN client c ON o.client_id = c.id
                JOIN order_status os ON o.order_status_id = os.id
            WHERE
                o.order_status_id IN ({})
            "#,
            placeholders
        );

        let mut query_builder = sqlx::query_as::<_, DbOrder>(&query);
        
        for id in &order_status_list {
            query_builder = query_builder.bind(id);
        }
        
        let result_order = query_builder
            .fetch_all(&*self.pool)
            .await;

        match result_order {
            Ok(vec) => {
                if vec.is_empty() {
                    Ok(None)
                } else {
                    let mut domain_orders: Vec<Order> = vec.into_iter().map(Into::into).collect();
                    for domain_order in &mut domain_orders {
                        let order_products = self.get_order_products(domain_order.id).await?;
                        domain_order.order_products = order_products;
                    }
                    Ok(Some(domain_orders))
                }
            },
            Err(e) => Err(Box::new(e))
        }
    }

    async fn create_order(&self, order: Order) -> Result<(), Box<dyn Error>> {
        let mut transaction = self.pool.begin().await?;

        let result = sqlx::query_as::<_, DbOrder> (
            r#"
            INSERT INTO orders (order_status_id, client_cpf, client_name, updated_at, created_at)
            OUTPUT INSERTED.id
            VALUES (
                @p1,
                @p2,
                @p3,
                GETDATE(),
                GETDATE()
            )
            "#
        )
        .bind(order.order_status.id)
        .bind(order.client.cpf)
        .bind(order.client.name)
        .fetch_one(&mut transaction)
        .await;

        match result {
            Ok(db_order) => {
                for order_product in &order.order_products {
                    let _ = sqlx::query(
                        r#"
                        INSERT INTO order_products (order_id, product_id, quantity, price, updated_at, created_at)
                        VALUES (
                            @p1,
                            @p2,
                            @p3,
                            @p4,
                            GETDATE(),
                            GETDATE()
                        )
                        "#
                    )
                    .bind(db_order.id)
                    .bind(order_product.product_id)
                    .bind(order_product.quantity)
                    .bind(order_product.price)
                    .execute(&mut transaction)
                    .await?;
                }
                transaction.commit().await?;
                Ok(())
            },
            Err(e) => {
                transaction.rollback().await?;
                Err(Box::new(e))
            }
        }
    }

    async fn update_order_status(&self, order_id: i32, order_status_id: i32) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            UPDATE orders
            SET order_status_id = @p1
            WHERE id = @p2
            "#
        )
        .bind(order_status_id)
        .bind(order_id)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),            
        }
    }
}
