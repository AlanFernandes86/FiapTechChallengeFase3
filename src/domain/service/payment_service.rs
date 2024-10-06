use std::error::Error;
use async_trait::async_trait;

use crate::domain::entities::order::Order;

#[async_trait]
pub trait PaymentService {
	async fn start_payment_by_qrcode(&self, order: &Order) -> Result<(), Box<dyn Error>>;
}