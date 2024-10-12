#[derive(serde::Serialize, Debug)]
pub struct Payment {
    pub id: Option<i32>,
    pub order_id: i32,
    pub payment_status_id: i32,
    pub payment_method_id: i32,
    pub payment_method_order_id: String,
    pub value: f64,
    pub origin: String,
    pub message: String,
    // pub updated_at: chrono::NaiveDateTime,
    // pub created_at: chrono::NaiveDateTime
}