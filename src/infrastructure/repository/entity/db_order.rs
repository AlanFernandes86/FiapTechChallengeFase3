use crate::domain::entities::{client::Client, order::{Order, OrderStatus}};

#[derive(sqlx::FromRow)]
pub struct DbOrder {
    pub id: i32,
    pub client_cpf: String,
    pub order_client_name: String,
    pub client_name: String,
    pub client_email: String,
    pub order_status_id: i32,
    pub order_status_name: String
    // pub updated_at: NaiveDateTime,
    // pub created_at: NaiveDateTime,
}

impl From<DbOrder> for Order {
    fn from(db_order: DbOrder) -> Self {
        Order {
            id: db_order.id,
            order_client_name: db_order.order_client_name,
            client: Client {
                cpf: db_order.client_cpf,
                name: db_order.client_name,
                email: db_order.client_email
            },
            order_status: OrderStatus {
                id: db_order.order_status_id,
                name: db_order.order_status_name
            },
            total: 0.0f64,
            order_products: vec![],
            // updated_at: db_order.updated_at,
            // created_at: db_order.created_at
        }
    }
}