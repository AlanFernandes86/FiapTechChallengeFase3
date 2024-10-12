use std::error::Error;
use crate::{
        application::order::update_order_status::UpdateOrderStatusUseCase, 
        controllers::models::payment::MercadoPagoNotificationDTO, domain::{
            entities::payment::Payment,
            enums::{order_status::OrderStatus, payment_method::PaymentMethod, payment_status::PaymentStatus},
            repository::payment_repository::PaymentRepository
        }, 
        infrastructure::service::mercado_pago::{mercado_pago_service::MercadoPagoService, merchant_order_response::MerchantOrderResponse}
    };

pub struct MercadoPagoNotificationUseCase {
    payment_repository: Box<dyn PaymentRepository>,
    updated_order_status_use_case: Box<UpdateOrderStatusUseCase>,
    mercado_pago_service: Box<MercadoPagoService>
}

impl MercadoPagoNotificationUseCase {
    pub fn new(
        payment_repository: Box<dyn PaymentRepository>,
        updated_order_status_use_case: Box<UpdateOrderStatusUseCase>,
        payment_service: Box<MercadoPagoService>
    ) -> Self {
        Self {
            payment_repository,
            updated_order_status_use_case,
            mercado_pago_service: payment_service,
        }
    }

    pub async fn handle(&self, start_payment_dto: &MercadoPagoNotificationDTO) -> Result<(), Box<dyn Error>> {
        let mp_token = self.mercado_pago_service.get_access_token().await?;

        if start_payment_dto.topic != "merchant_order" {
            return Ok(());
        }

        let merchant_order = self.mercado_pago_service.get_resource::<MerchantOrderResponse>(&start_payment_dto.resource, &mp_token).await?;

        match merchant_order.status.as_str() {
            "opened" => {
                self.pending_payment_in_database(&merchant_order).await?;
            },
            "closed" => {
                self.paid_payment_in_database(&merchant_order).await?;
                let (_, order_id) = self.get_origin_and_order_id(&merchant_order.external_reference);
                let update_order_status_result = self.updated_order_status_use_case.handle(order_id, OrderStatus::Received as i32).await;
                match update_order_status_result {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }
            },
            "cancelled" => {
                self.cancelled_payment_in_database(&merchant_order).await?;
            },
            _ => {}
        }

        Ok(())
    }

    async fn pending_payment_in_database(&self, merchant_order: &MerchantOrderResponse) -> Result<(), Box<dyn Error>> {
      let (origin, order_id) = self.get_origin_and_order_id(&merchant_order.external_reference);
        let payment = Payment {
            id: None,
            order_id: order_id,
            payment_status_id: PaymentStatus::Pending as i32,
            payment_method_id: PaymentMethod::MercadoPago as i32,
            payment_method_order_id: merchant_order.id.to_string(),
            value: merchant_order.total_amount,
            origin: origin,
            message: "Pagamento da ordem id: ".to_string() 
            + &order_id.to_string() 
            + " iniciado com sucesso. MercadoPago Order ID: " 
            + &merchant_order.id.to_string(),
        };
        self.payment_repository.put_payment(payment).await?;
        Ok(())
    }

    async fn paid_payment_in_database(&self, merchant_order: &MerchantOrderResponse) -> Result<(), Box<dyn Error>> {
        let (origin, order_id) = self.get_origin_and_order_id(&merchant_order.external_reference);
        let payment = Payment {
            id: None,
            order_id: order_id,
            payment_status_id: PaymentStatus::Paid as i32,
            payment_method_id: PaymentMethod::MercadoPago as i32,
            payment_method_order_id: merchant_order.id.to_string(),
            value: merchant_order.total_amount,
            origin: origin,
            message: "Pagamento da ordem id: ".to_string() 
            + &order_id.to_string() 
            + " realizado com sucesso. MercadoPago Order ID: " 
            + &merchant_order.id.to_string(),
        };
        self.payment_repository.put_payment(payment).await?;
        Ok(())
    }

    async fn cancelled_payment_in_database(&self, merchant_order: &MerchantOrderResponse) -> Result<(), Box<dyn Error>> {
      let (origin, order_id) = self.get_origin_and_order_id(&merchant_order.external_reference);
      let payment = Payment {
          id: None,
          order_id: order_id,
          payment_status_id: PaymentStatus::Cancelled as i32,
          payment_method_id: PaymentMethod::MercadoPago as i32,
          payment_method_order_id: merchant_order.id.to_string(),
          value: merchant_order.total_amount,
          origin,
          message: "Pagamento da ordem id: ".to_string() 
          + &order_id.to_string() 
          + " cancelado! MercadoPago Order ID: " 
          + &merchant_order.id.to_string()
          + ". Motivo: "
          + &merchant_order.additional_info.to_string(),
      };
      self.payment_repository.put_payment(payment).await?;
      Ok(())
    }

    fn get_origin_and_order_id(&self, external_reference: &str) -> (String, i32) {
        let parts: Vec<&str> = external_reference.split('#').collect();
        if parts.len() != 2 {
          return ("".to_string(), 0);
        }
        let origin = parts[0].to_string();
        let order_id = parts[1].parse::<i32>().unwrap_or(0);
        (origin, order_id)
    }
}