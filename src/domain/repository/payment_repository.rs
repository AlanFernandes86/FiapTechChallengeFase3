use std::error::Error;
use async_trait::async_trait;
use crate::domain::entities::payment::Payment;

#[async_trait]
pub trait PaymentRepository {
    async fn put_payment(&self, payment: Payment) -> Result<(), Box<dyn Error>>;
}