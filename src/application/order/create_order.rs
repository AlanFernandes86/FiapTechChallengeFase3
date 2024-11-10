use std::error::Error;

use crate::domain::{
    entities::order::Order,
    enums::order_status::EnOrderStatus,
    repository::{client_repository::ClientRepository, order_repository::OrderRepository, product_repository::ProductRepository}
};

pub struct CreateOrderUseCase {
    order_repository: Box<dyn OrderRepository>,
    client_repository: Box<dyn ClientRepository>,
    product_repository: Box<dyn ProductRepository>,
}

impl CreateOrderUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>, client_repository: Box<dyn ClientRepository>, product_repository: Box<dyn ProductRepository>) -> Self {
        Self {
            order_repository,
            client_repository,
            product_repository
        }
    }

    pub async fn handle(&self, mut order: Order) -> Result<i32, Box<dyn Error>> {
        if order.order_status.id != EnOrderStatus::Created as i32 {
            return Err("order_status_id invalid! It is only possible to create orders with the status CREATED.".into());
        }

        if order.client.cpf.len() == 11 {
            let client_result = self.client_repository.get_client_by_cpf(order.client.cpf).await;
            let client = match client_result {
                Ok(client) => client.unwrap(),
                Err(_) => return Err("Client not found!".into())
            };

            order.client = client;
        }

        for order_product in order.order_products.iter_mut() {
            let product_result = self.product_repository.get_product_by_id(order_product.product_id).await;
            let product = match product_result {
                Ok(product) => product.unwrap(),
                Err(e) => return Err(format!("Error getting product by id: {:?}", e).into())
            };
            
            order_product.name = product.name;
            order_product.description = product.description;
            order_product.image_url = product.image_url;
            order_product.product_category = product.product_category;
        }

        self.order_repository.create_order(order).await
    }
}