use std::fmt;

#[derive(Debug)]
pub enum EnOrderStatus {
    Created = 1,
    Received = 2,
    InPreparation = 4,
    Ready = 8,
    Completed = 16,
    Cancelled = 32,
    Active = 14 // only for search purposes [Received, InPreparation, Ready]
}

impl EnOrderStatus {
    pub fn from_id(id: i32) -> Result<EnOrderStatus, String> {
        match id {
            1 => Ok(EnOrderStatus::Created),
            2 => Ok(EnOrderStatus::Received),
            4 => Ok(EnOrderStatus::InPreparation),
            8 => Ok(EnOrderStatus::Ready),
            16 => Ok(EnOrderStatus::Completed),
            32 => Ok(EnOrderStatus::Cancelled),
            _ => Err(format!("OrderStatus with id [{}] does not exist.", id))     
        }
    }
}

impl fmt::Display for EnOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnOrderStatus::Created => write!(f, "Created"),
            EnOrderStatus::Received => write!(f, "Received"),
            EnOrderStatus::InPreparation => write!(f, "InPreparation"),
            EnOrderStatus::Ready => write!(f, "Ready"),
            EnOrderStatus::Completed => write!(f, "Completed"),
            EnOrderStatus::Cancelled => write!(f, "Cancelled"),
            EnOrderStatus::Active => write!(f, "Active")
        }
    }
}