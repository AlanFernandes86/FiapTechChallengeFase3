#[derive(serde::Deserialize, Debug)]
pub struct PutOrderProductDTO {
    pub order_id: i32,
    pub product_id: i32,
    pub price: f32,
    pub quantity: i32
}