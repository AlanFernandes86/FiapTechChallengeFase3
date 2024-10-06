use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMpOrderRequest {
    pub external_reference: String,
    pub title: String,
    pub description: String,
    pub notification_url: String,
    pub total_amount: f64,
    pub items: Vec<CreateMpOrderRequestItem>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMpOrderRequestItem {
    pub title: String,
    pub description: String,
    pub unit_price: f64,
    pub quantity: i64,
    pub unit_measure: String,
    pub total_amount: f64,
}