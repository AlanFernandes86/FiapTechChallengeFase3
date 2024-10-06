use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantOrderResponse {
    pub id: i64,
    pub status: String,
    pub external_reference: String,
    pub preference_id: String,
    pub payments: Vec<Payment>,
    pub shipments: Vec<Value>,
    pub payouts: Vec<Value>,
    pub collector: Collector,
    pub marketplace: String,
    pub notification_url: String,
    pub date_created: String,
    pub last_updated: String,
    pub sponsor_id: String,
    pub shipping_cost: i64,
    pub total_amount: f64,
    pub site_id: String,
    pub paid_amount: f64,
    pub refunded_amount: i64,
    pub payer: Payer,
    pub items: Vec<Item>,
    pub cancelled: bool,
    pub additional_info: String,
    pub application_id: String,
    pub is_test: bool,
    pub order_status: String,
    pub client_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub transaction_amount: f64,
    pub total_paid_amount: f64,
    pub shipping_cost: i64,
    pub currency_id: String,
    pub status: String,
    pub status_detail: String,
    pub operation_type: String,
    pub date_approved: String,
    pub date_created: String,
    pub last_modified: String,
    pub amount_refunded: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collector {
    pub id: i64,
    pub email: String,
    pub nickname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payer {
    pub id: i64,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub category_id: String,
    pub currency_id: String,
    pub description: String,
    pub picture_url: String,
    pub title: String,
    pub quantity: i64,
    pub unit_price: f64,
}
