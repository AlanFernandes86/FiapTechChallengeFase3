use std::error::Error;
use crate::{
        controllers::models::payment::StartPaymentDTO, domain::{enums::payment_status::EnPaymentStatus, repository::order_repository::OrderRepository, service::{models::start_payment_response::StartPaymentResponse, payment_service::PaymentService}}
    };

pub struct StartPaymentUseCase {
    order_repository: Box<dyn OrderRepository>,
    payment_service: Box<dyn PaymentService>
}

impl StartPaymentUseCase {
    pub fn new(
        order_repository: Box<dyn OrderRepository>,
        payment_service: Box<dyn PaymentService>
    ) -> Self {
        Self {
            order_repository,
            payment_service,
        }
    }

    pub async fn handle(&self, start_payment_dto: StartPaymentDTO) -> Result<Option<StartPaymentResponse>, Box<dyn Error>> {
        let result_get_order = self.order_repository.get_order_by_id(start_payment_dto.order_id).await;
        match  result_get_order {
            Ok(Some(order)) => {
                if order.order_payment.id.is_some_and(|id| id == EnPaymentStatus::Paid as i32) {
                    return Err("Order already paid".into());
                }

                let start_payment_result = self.payment_service.start_payment(&order, start_payment_dto.pdv_id.clone()).await?;
                Ok(Some(start_payment_result))
            },
            Ok(None) => Ok(None),            
            Err(e) => return Err(e)
        }
    }
}