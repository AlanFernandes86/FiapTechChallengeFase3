use aws_sdk_dynamodb::types::AttributeValue;
use crate::domain::entities::order::{Order, OrderPayment, OrderStatus};
use crate::domain::entities::user::User;
use crate::domain::entities::order_product::OrderProduct;
use crate::domain::enums::order_status::EnOrderStatus;
use crate::domain::enums::payment_status::EnPaymentStatus;
use super::db_order_product::DbOrderProduct;

pub struct DbOrder {
    pub order_id: i32,
    pub order_name: String,
    pub client_name: String,
    pub client_cpf: String,
    pub client_email: String,
    pub user_group: String,
    pub order_status_id: i32,
    pub order_payment_status_id: Option<i32>,
    pub order_products: Vec<DbOrderProduct>
    // pub updated_at: NaiveDateTime,
    // pub created_at: NaiveDateTime,
}

impl DbOrder {
    pub fn from_item(item: std::collections::HashMap<String, AttributeValue>) -> Self {
        let order_id = item.get("order_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let order_client_name = item.get("order_name")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let client_name = item.get("client_name")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let client_cpf = item.get("client_cpf")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let client_email = item.get("client_email")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let user_group = item.get("user_group")
            .and_then(|v| v.as_n().ok())
            .map(|v| v.to_string())
            .unwrap_or_default();       

        let order_status_id = item.get("order_status_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let order_payment_status_id = item.get("order_payment_status_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok());

        let order_products = item.get("order_products")
            .and_then(|v| v.as_m().ok())
            .map(|v| {
                v.iter().map(|item| {
                    DbOrderProduct::from_item(item.1.as_m().ok().unwrap().clone())
                }).collect()
            })
            .unwrap_or_default();

        Self { order_id, client_name, order_name: order_client_name, client_cpf, client_email, user_group, order_status_id, order_payment_status_id, order_products }
    }    
}

impl From<DbOrder> for Order {
    fn from(db_order: DbOrder) -> Self {
        Order {
            id: db_order.order_id,
            order_name: db_order.order_name,
            client: User {
                cpf: db_order.client_cpf,
                name: db_order.client_name,
                email: db_order.client_email,
                group: db_order.user_group
            },
            order_status: OrderStatus {
                id: db_order.order_status_id,
                name: EnOrderStatus::from_id(db_order.order_status_id).unwrap().to_string()
            },
            order_payment: OrderPayment{
                id: db_order.order_payment_status_id,
                name: db_order.order_payment_status_id.map(|id| EnPaymentStatus::from_id(Some(id)).unwrap().to_string())
            },
            total: 0.0f64,
            order_products: db_order.order_products.into_iter().map(OrderProduct::from).collect(),
            // updated_at: db_order.updated_at,
            // created_at: db_order.created_at
        }
    }
}

