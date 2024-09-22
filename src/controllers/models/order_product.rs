#[derive(serde::Deserialize, Debug)]
pub struct PutOrderProductDTO {
    pub order_product_id: Option<i32>,
    pub order_id: i32,
    pub product_id: i32,
    pub price: f32,
    pub quantity: i32
}