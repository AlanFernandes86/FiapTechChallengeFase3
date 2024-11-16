use std::error::Error;

use crate::{
    domain::{
        entities::order::Order,
        enums::order_status::EnOrderStatus,
        repository::{user_repository::UserRepository, order_repository::OrderRepository, product_repository::ProductRepository}
    }, 
    infrastructure::repository::memory_cache::memory_cache::MemoryCache
};

pub struct CreateOrderUseCase<'a> {
    order_repository: Box<dyn OrderRepository>,
    client_repository: Box<dyn UserRepository>,
    product_repository: Box<dyn ProductRepository>,
    memory_cache: &'a MemoryCache
}

impl<'a> CreateOrderUseCase<'a> {
    pub fn new(order_repository: Box<dyn OrderRepository>, client_repository: Box<dyn UserRepository>, product_repository: Box<dyn ProductRepository>) -> Self {
        let memory_cache = MemoryCache::instance();
        Self {
            order_repository,
            client_repository,
            product_repository,
            memory_cache
        }
    }

    pub async fn handle(&self, mut order: Order) -> Result<i32, Box<dyn Error>> {
        if order.order_status.id != EnOrderStatus::Created as i32 {
            return Err("order_status_id invalid! It is only possible to create orders with the status CREATED.".into());
        }

        if order.client.cpf.len() == 11 {
            let client_result = self.client_repository.get_user_by_cpf(order.client.cpf).await;
            let client = match client_result {
                Ok(client) => client.unwrap(),
                Err(_) => return Err("Client not found!".into())
            };

            order.client = client;
        }

        let products_map = self.memory_cache.get("products".to_string());
        let products = match products_map {
            Some(products) => products,
            None => {
                let products_result = self.product_repository.get_products().await?;
                match products_result {
                    Some(products) => {
                        self.memory_cache.set("products".to_string(), products.clone());
                        products
                    },
                    None => return Err("Products not found!".into())                    
                }
            }
        };
        
        for order_product in order.order_products.iter_mut() {
            let id = order_product.product_id;
            let product_option = products.get(&id);
            match product_option {
                Some(product) => {
                    order_product.name = product.name.clone();
                    order_product.description = product.description.clone();
                    order_product.image_url = product.image_url.clone();
                    order_product.product_category = product.product_category.clone();
                },
                None => {
                    return Err("Product not found!".into());
                }                
            }            
        }

        self.order_repository.create_order(order).await
    }
}