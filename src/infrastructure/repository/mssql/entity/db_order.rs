use crate::domain::entities::{order::{Order, OrderPayment, OrderStatus}, user::User};

#[derive(sqlx::FromRow)]
pub struct DbOrder {
    pub id: i32,
    pub client_cpf: String,
    pub order_client_name: String,
    pub client_name: String,
    pub client_email: String,
    pub order_status_id: i32,
    pub order_status_name: String,
    pub payment_status_id: Option<i32>,
    pub payment_status_name: Option<String>,
    // pub updated_at: NaiveDateTime,
    // pub created_at: NaiveDateTime,
}

impl From<DbOrder> for Order {
    fn from(db_order: DbOrder) -> Self {
        Order {
            id: db_order.id,
            order_name: db_order.order_client_name,
            client: User {
                cpf: db_order.client_cpf,
                name: db_order.client_name,
                email: db_order.client_email,
                group: "".to_string()
            },
            order_status: OrderStatus {
                id: db_order.order_status_id,
                name: db_order.order_status_name
            },
            order_payment: OrderPayment{
                id: db_order.payment_status_id,
                name: db_order.payment_status_name
            },
            total: 0.0f64,
            order_products: vec![],
            // updated_at: db_order.updated_at,
            // created_at: db_order.created_at
        }
    }
}