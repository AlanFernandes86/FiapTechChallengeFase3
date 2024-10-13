use std::{error::Error, sync::Arc};
use crate::domain::{errors::invalid_order_status_update_error::InvalidOrderStatusUpdateError, messaging::event_publisher::EventPublisher, repository::{order_product_repository::OrderProductRepository, order_repository::OrderRepository}};

pub struct UpdateOrderStatusUseCase {
    order_repository: Arc<dyn OrderRepository>,
    order_product_repository: Arc<dyn OrderProductRepository>,
    message_publisher: Arc<dyn EventPublisher>
}

impl UpdateOrderStatusUseCase {
    pub fn new(
        order_repository: Arc<dyn OrderRepository>,
        order_product_repository: Arc<dyn OrderProductRepository>,
        message_publisher: Arc<dyn EventPublisher>
    ) -> Self {
        Self {
            order_repository,
            order_product_repository,
            message_publisher
        }
    }

    pub async fn handle(&self, order_id: i32, order_status_id: i32) -> Result<Option<()>, Box<dyn Error + Send + Sync>> {
        let order_as_is_option = self.order_repository.get_order_by_id(order_id).await?;
        if let Some(order_as_is) = order_as_is_option {
            if order_as_is.is_this_valid_status_update(order_status_id) {
                self.order_repository.update_order_status(order_id, order_status_id).await?;
                self.publish_order_status_update(order_id);            
                return Ok(Some(()));
            }
            else {
                return Err(Box::new(InvalidOrderStatusUpdateError::new(order_status_id)));
            }
        }

        Ok(None)
    }

    fn publish_order_status_update(&self, order_id: i32) {
        // Faz o get e publish em uma nova thread, não bloqueando a execução
        let repository = self.order_repository.clone();
        let order_product_repository = self.order_product_repository.clone();
        let message_publisher = self.message_publisher.clone();
        tokio::spawn(async move {
            let result = repository.get_order_by_id(order_id).await;
            match result {
                Ok(Some(mut order)) => {
                    let order_products = order_product_repository.get_order_products(order_id).await.unwrap();
                    order.order_products = order_products;
                    order.total = order.calculate_total();
                    message_publisher.publish_order_status_update(&order).await;                                        
                },
                Ok(None) => {},
                Err(_) => {}
            }
        });
    }
}
