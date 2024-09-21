#[derive(Debug)]
pub enum OrderStatus {
    Created = 1,
    Received = 2,
    InPreparation = 4,
    Ready = 8,
    Completed = 16,
    Cancelled = 32,
    Active = 14
}

impl OrderStatus {
    pub fn from_id(id: i32) -> Result<OrderStatus, String> {
        match id {
            1 => Ok(OrderStatus::Created),
            2 => Ok(OrderStatus::Received),
            4 => Ok(OrderStatus::InPreparation),
            8 => Ok(OrderStatus::Ready),
            16 => Ok(OrderStatus::Completed),
            32 => Ok(OrderStatus::Cancelled),
            14 => Ok(OrderStatus::Active),
            _ => Err(format!("OrderStatus with id [{}] does not exist.", id))     
        }
    }
}