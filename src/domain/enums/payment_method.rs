#[derive(Debug)]
pub enum PaymentMethod {
    MercadoPago = 1,
    PicPay = 2
}

impl PaymentMethod {
    pub fn from_id(id: i32) -> Result<PaymentMethod, String> {
        match id {
            1 => Ok(PaymentMethod::MercadoPago),
            2 => Ok(PaymentMethod::PicPay),
            _ => Err(format!("PaymentMethod with id [{}] does not exist.", id))     
        }
    }
}