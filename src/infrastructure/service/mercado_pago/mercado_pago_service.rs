use std::error::Error;
use async_trait::async_trait;
use crate::domain::{entities::order::Order, service::payment_service::PaymentService};

pub struct MercadoPagoService {
    http_client: reqwest::Client,
}

impl MercadoPagoService {
    // O método new é definido diretamente na struct
    pub fn new(http_client: reqwest::Client) -> Self {
        MercadoPagoService { http_client }
    }
}

#[async_trait]
impl PaymentService for MercadoPagoService {
    async fn start_payment_by_qrcode(&self, order: &Order) -> Result<(), Box<dyn Error>> {
        print!("start_payment_by_qrcode");
        Ok(())
    }
}