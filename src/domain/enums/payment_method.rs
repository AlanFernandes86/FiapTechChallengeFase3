#[derive(Debug)]
pub enum EnPaymentMethod {
    MercadoPago = 1,
    PicPay = 2
}

impl EnPaymentMethod {
    pub fn from_id(id: i32) -> Result<EnPaymentMethod, String> {
        match id {
            1 => Ok(EnPaymentMethod::MercadoPago),
            2 => Ok(EnPaymentMethod::PicPay),
            _ => Err(format!("PaymentMethod with id [{}] does not exist.", id))     
        }
    }
}