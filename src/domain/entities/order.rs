use super::{client::Client, product_category::ProductCategory};

#[derive(serde::Serialize, Debug)]
pub struct Order {
    pub id: i32,
    pub client: Client,
    pub order_status: OrderStatus,
    pub total: f64,
    pub order_products: Vec<OrderProduct>,
    // pub updated_at: chrono::NaiveDateTime,
    // pub created_at: chrono::NaiveDateTime
}

#[derive(serde::Serialize, Debug)]
pub struct OrderStatus {
    pub id: i32,
    pub name: String
}

#[derive(serde::Serialize, Debug)]
pub struct OrderProduct {
    pub order_product_id: i32,
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