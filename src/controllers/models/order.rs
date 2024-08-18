use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetOrdersQuery {
    order_status: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateOrderStatusDTO {
    pub order_id: i32,
    pub order_status: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateOrderDTO {
    pub client_cpf: i32,
    pub client_name: i32,
    pub products: Vec<OrderProductDTO>
}

#[derive(serde::Deserialize, Debug)]
pub struct OrderProductDTO {
    pub product_id: i32,
    pub price: f32,
    pub quantity: i32
}