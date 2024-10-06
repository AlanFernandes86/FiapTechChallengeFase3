use std::{error::Error, sync::Arc};
use async_trait::async_trait;
use sqlx::MssqlPool;

use crate::domain::{entities::payment::Payment, repository::payment_repository::PaymentRepository};

pub struct MssqlPaymentRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlPaymentRepository {
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlPaymentRepository { pool }
    }
}

#[async_trait]
impl PaymentRepository for MssqlPaymentRepository {
    async fn put_payment(&self, payment: Payment) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            "MERGE INTO TechChallenge.dbo.payment AS target
            USING (VALUES (@p1, @p2, @p3, @p4, @p5, @p6, GETDATE(), GETDATE())) AS source 
            (id, order_id, payment_status_id, payment_method_id, value, message, updated_at, created_at)
            ON target.id = source.id
            WHEN MATCHED THEN
                UPDATE SET
                    target.payment_status_id = source.payment_status_id,
                    target.value = source.value,
                    target.message = source.message,
                    target.updated_at = source.updated_at
            WHEN NOT MATCHED THEN
                INSERT (order_id, payment_status_id, payment_method_id, value, message, updated_at, created_at)
                VALUES (source.order_id, source.payment_status_id, source.payment_method_id, source.value, source.message, source.updated_at, source.created_at);"
        )
        .bind(payment.id)
        .bind(payment.order_id)
        .bind(payment.payment_status_id)
        .bind(payment.payment_method_id)
        .bind(payment.value)
        .bind(payment.message)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}