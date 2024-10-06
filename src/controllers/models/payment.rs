#[derive(serde::Deserialize, Debug)]
pub struct StartPaymentDTO {
    pub order_id: i32,
    pub pdv_id: String,
    pub payment_method_id: i32
}

pub struct MercadoPagoNotificationDTO {
    pub topic: String,
    pub resource: String
}