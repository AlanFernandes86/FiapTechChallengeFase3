use std::error::Error;
use async_trait::async_trait;

use crate::domain::entities::order::Order;
use crate::domain::service::models::start_payment_response::StartPaymentResponse;

#[async_trait]
pub trait PaymentService {
	async fn start_payment(&self, order: &Order, pdv_id: String) -> Result<StartPaymentResponse, Box<dyn Error>>;
}