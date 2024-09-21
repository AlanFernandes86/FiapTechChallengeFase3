use std::error::Error;
use crate::domain::{errors::invalid_order_status_update_error::InvalidOrderStatusUpdateError, repositories::order_repository::OrderRepository};

pub struct UpdateOrderStatusUseCase {
    order_repository: Box<dyn OrderRepository>
}

impl UpdateOrderStatusUseCase {
    pub fn new(order_repository: Box<dyn OrderRepository>) -> Self {
        Self {
            order_repository,
        }
    }

    pub async fn handle(&self, order_id: i32, order_status_id: i32) -> Result<Option<()>, Box<dyn Error>> {
        let order_as_is_option = self.order_repository.get_order_by_id(order_id).await?;
        if let Some(order_as_is) = order_as_is_option {
            if order_as_is.is_this_valid_status_update(order_status_id) {
                self.order_repository.update_order_status(order_id, order_status_id).await?;
                return Ok(Some(()));
            }
            else {
                return Err(Box::new(InvalidOrderStatusUpdateError::new(order_status_id)));
            }
        }

        Ok(None)
    }
}
