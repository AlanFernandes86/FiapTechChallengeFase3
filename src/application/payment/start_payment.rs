use std::error::Error;
use crate::{
        application::order::get_order_by_id::GetOrderByIdUseCase, 
        controllers::models::payment::StartPaymentDTO, domain::{
            entities::{order::Order, payment::Payment},
            enums::payment_status::PaymentStatus,
            repository::payment_repository::PaymentRepository,
            service::payment_service::PaymentService
        }
    };

pub struct StartPaymentUseCase {
    payment_repository: Box<dyn PaymentRepository>,
    get_order_by_id_use_case: Box<GetOrderByIdUseCase>,
    payment_service: Box<dyn PaymentService>
}

impl StartPaymentUseCase {
    pub fn new(
        payment_repository: Box<dyn PaymentRepository>,
        get_order_by_id_use_case: Box<GetOrderByIdUseCase>,
        payment_service: Box<dyn PaymentService>
    ) -> Self {
        Self {
            payment_repository,
            get_order_by_id_use_case,
            payment_service,
        }
    }

    pub async fn handle(&self, start_payment_dto: StartPaymentDTO) -> Result<Option<()>, Box<dyn Error>> {
        let result_get_order = self.get_order_by_id_use_case.handle(start_payment_dto.order_id).await;
        match  result_get_order {
            Ok(Some(order)) => {
                self.payment_service.start_payment_by_qrcode(&order).await?;
                self.create_payment_in_database(start_payment_dto, &order).await?;
                Ok(Some(()))
            },
            Ok(None) => Ok(None),            
            Err(e) => return Err(e)
        }
    }

async fn create_payment_in_database(&self, payment_dto: StartPaymentDTO, order: &Order) -> Result<(), Box<dyn Error>> {
        let payment = Payment {
            id: None,
            order_id: payment_dto.order_id,
            payment_status_id: PaymentStatus::Pending as i32,
            payment_method_id: payment_dto.payment_method_id,
            value: order.total,
            message: "Pagamento da ordem id: ".to_string() + &payment_dto.order_id.to_string() + " iniciado com sucesso.",
        };
        self.payment_repository.put_payment(payment).await?;
        Ok(())
    }
}