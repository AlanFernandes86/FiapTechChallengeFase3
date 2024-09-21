use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InvalidOrderStatusUpdateError
{
    pub invalid_status_id: i32,
}

impl InvalidOrderStatusUpdateError {
    pub fn new(invalid_status_id: i32) -> Self {
        InvalidOrderStatusUpdateError { invalid_status_id }
    }
}

impl fmt::Display for InvalidOrderStatusUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "update_status_id [{}] provided does not exist, is equal to, or is not a later status than the current one.", self.invalid_status_id)
    }
}

impl Error for InvalidOrderStatusUpdateError {}