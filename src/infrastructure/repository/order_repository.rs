use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::order::Order;
use crate::domain::repositories::order_repository::OrderRepository;
use crate::infrastructure::repository::entity::db_order::DbOrder;

pub struct MssqlOrderRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlOrderRepository {
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlOrderRepository { pool }
    }
}

#[async_trait]
impl OrderRepository for MssqlOrderRepository {
    async fn get_order_by_id(&self, order_id: i32) -> Result<Option<Order>, Box<dyn Error>> {
        let result_order = sqlx::query_as::<_, DbOrder>(
            r#"
            SELECT
                o.id,
                o.client_name AS order_client_name,
                c.cpf AS client_cpf,
                c.name AS client_name,
                c.email AS client_email,
                os.id AS order_status_id,
                os.name AS order_status_name
            FROM
                TechChallenge.dbo.[order] o
                JOIN TechChallenge.dbo.client c ON o.client_cpf = c.cpf
                JOIN TechChallenge.dbo.order_status os ON o.order_status_id = os.id
            WHERE
                o.id = @p1
            "#
        )
        .bind(order_id)
        .fetch_optional(&*self.pool)
        .await;

        match result_order {
            Ok(option) => {
                match option {
                    Some(db_order) => Ok(Some(db_order.into())),
                    None => Ok(None)
                }
            },
            Err(e) => Err(Box::new(e))
        }        
    }

    async fn get_orders_by_status(&self, order_status_list: Vec<i32>) -> Result<Option<Vec<Order>>, Box<dyn Error>> {
        let placeholders = order_status_list
        .iter()
        .enumerate() // Adiciona o índice ao iterador
        .map(|(i, _)| format!("@p{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");

        // Cria a query SQL com base na lista de status
        let query = format!(
            r#"
            SELECT
                o.id,
                o.client_name AS order_client_name,
                c.cpf AS client_cpf,
                c.name AS client_name,
                c.email AS client_email,
                os.id AS order_status_id,
                os.name AS order_status_name,
                o.updated_at,
                o.created_at
            FROM
                TechChallenge.dbo.[order] o
                JOIN TechChallenge.dbo.client c ON o.client_cpf = c.cpf
                JOIN TechChallenge.dbo.order_status os ON o.order_status_id = os.id
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
            Ok(option) => {
                match option {
                    vec if vec.is_empty() => Ok(None),
                    vec => Ok(Some(vec.into_iter().map(Into::into).collect()))
                }
            },
            Err(e) => Err(Box::new(e))
        }
    }

    async fn create_order(&self, order: Order) -> Result<i32, Box<dyn Error>> {
        let mut transaction = self.pool.begin().await?;

        let result: Result<i32, sqlx::Error> = sqlx::query_scalar(
            r#"
            INSERT INTO TechChallenge.dbo.[order] (order_status_id, client_cpf, client_name, updated_at, created_at)
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
        .bind(order.order_client_name)
        .fetch_one(&mut transaction)
        .await;

        match result {
            Ok(db_order_id) => {
                for order_product in &order.order_products {
                    let _ = sqlx::query(
                        r#"
                        INSERT INTO TechChallenge.dbo.[order_product] (order_id, product_id, quantity, price, updated_at, created_at)
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
                    .bind(db_order_id)
                    .bind(order_product.product_id)
                    .bind(order_product.quantity)
                    .bind(order_product.price)
                    .execute(&mut transaction)
                    .await?;
                }
                transaction.commit().await?;
                Ok(db_order_id)
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
            UPDATE TechChallenge.dbo.[order]
            SET order_status_id = @p1,
            updated_at = GETDATE()
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
