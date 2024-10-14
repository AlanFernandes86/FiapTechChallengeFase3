use super::product_category::ProductCategory;

#[derive(serde::Serialize, Debug)]
pub struct OrderProduct {
    pub order_product_id: Option<i32>,
    pub order_id: i32,
    pub product_id: i32,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub description: String,
    pub image_url: String,
    pub product_category: ProductCategory,
    // pub updated_at: chrono::NaiveDateTime,
    // pub created_at: chrono::NaiveDateTime
}