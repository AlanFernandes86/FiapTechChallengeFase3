use std::fmt;

#[derive(Debug)]
pub enum EnPaymentStatus {
    Pending = 1,
    Paid = 2,
    Cancelled = 4
}

impl EnPaymentStatus {
    pub fn from_id(id: Option<i32>) -> Result<Self, String> {
        match id {
            Some(1) => Ok(Self::Pending),
            Some(2) => Ok(Self::Paid),
            Some(4) => Ok(Self::Cancelled),
            _ => Err("Invalid payment status id".to_string())
        }
    }
}

impl fmt::Display for EnPaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Pending => write!(f, "Pending"),
            Self::Paid => write!(f, "Paid"),
            Self::Cancelled => write!(f, "Cancelled")
        }
    }
}