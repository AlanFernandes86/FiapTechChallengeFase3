use serde::Deserialize;
use crate::domain::entities::{user::User, order::Order, order_product::OrderProduct, product_category::ProductCategory};

#[derive(Deserialize, Debug)]
pub struct GetOrdersQuery {
    pub order_status_id: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateOrderStatusDTO {
    pub order_id: i32,
    pub order_status_id: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateOrderDTO {
    pub client_cpf: Option<String>,
    pub client_name: String,
    pub products: Vec<OrderProductDTO>
}

#[derive(serde::Deserialize, Debug)]
pub struct OrderProductDTO {
    pub product_id: i32,
    pub price: f64,
    pub quantity: i32
}

impl From<CreateOrderDTO> for Order {
    fn from(client_dto: CreateOrderDTO) -> Self {
        Order {
            id: 0,
            order_name: client_dto.client_name.clone(),
            order_status: crate::domain::entities::order::OrderStatus {
                id: crate::domain::enums::order_status::EnOrderStatus::Created as i32,
                name: format!("{:?}", crate::domain::enums::order_status::EnOrderStatus::Created)
            },
            order_payment: crate::domain::entities::order::OrderPayment {
                id: None,
                name: None
            },
            client: User {
                cpf: client_dto.client_cpf.clone().unwrap_or_default(),
                name: client_dto.client_name,
                email: "".to_string(),
                group: "".to_string()
            },
            order_products: client_dto.products.into_iter().map(|product| OrderProduct {
                order_product_id: None,
                order_id: 0,
                name: "".to_string(),
                description: "".to_string(),
                image_url: "".to_string(),
                product_category: ProductCategory {
                    id: 0,
                    name: "".to_string(),
                    description: "".to_string()
                },
                product_id: product.product_id,
                price: product.price,
                quantity: product.quantity
            }).collect(),
            total: 0.0
        }
    }
}