use super::{client::Client, order_product::OrderProduct};

#[derive(serde::Serialize, Debug)]
pub struct Order {
    pub id: i32,
    pub order_client_name: String,
    pub client: Client,
    pub order_status: OrderStatus,
    pub order_payment: OrderPayment,
    pub total: f64,
    pub order_products: Vec<OrderProduct>,
    // pub updated_at: chrono::NaiveDateTime,
    // pub created_at: chrono::NaiveDateTime
}

impl Order {
    pub fn is_this_valid_status_update(&self, id: i32) -> bool {
        self.order_status.id != id
        && self.order_status.id < id
        && crate::domain::enums::order_status::EnOrderStatus::from_id(id).is_ok()
    }

    pub fn calculate_total(&self) -> f64 {
        (self.order_products.iter().fold(0.0, |acc, op| acc + op.price * op.quantity as f64) * 100.0).round() / 100.0
    }
}

#[derive(serde::Serialize, Debug)]
pub struct OrderStatus {
    pub id: i32,
    pub name: String
}

#[derive(serde::Serialize, Debug)]
pub struct OrderPayment {
    pub id: Option<i32>,
    pub name: Option<String>    
}
