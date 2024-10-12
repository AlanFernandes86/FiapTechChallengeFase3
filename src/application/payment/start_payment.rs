use std::error::Error;
use crate::{
        application::order::get_order_by_id::GetOrderByIdUseCase, 
        controllers::models::payment::StartPaymentDTO, domain::service::{models::start_payment_response::StartPaymentResponse, payment_service::PaymentService}
    };

pub struct StartPaymentUseCase {
    get_order_by_id_use_case: Box<GetOrderByIdUseCase>,
    payment_service: Box<dyn PaymentService>
}

impl StartPaymentUseCase {
    pub fn new(
        get_order_by_id_use_case: Box<GetOrderByIdUseCase>,
        payment_service: Box<dyn PaymentService>
    ) -> Self {
        Self {
            get_order_by_id_use_case,
            payment_service,
        }
    }

    pub async fn handle(&self, start_payment_dto: StartPaymentDTO) -> Result<Option<StartPaymentResponse>, Box<dyn Error>> {
        let result_get_order = self.get_order_by_id_use_case.handle(start_payment_dto.order_id).await;
        match  result_get_order {
            Ok(Some(order)) => {
                let start_payment_result = self.payment_service.start_payment(&order, start_payment_dto.pdv_id.clone()).await?;
                Ok(Some(start_payment_result))
            },
            Ok(None) => Ok(None),            
            Err(e) => return Err(e)
        }
    }
}