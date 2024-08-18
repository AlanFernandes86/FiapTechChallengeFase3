#[derive(serde::Deserialize, Debug)]
pub struct SetPaymentDTO {
    pub order_id: i32,
    pub value: i32,
    pub method: f32
}