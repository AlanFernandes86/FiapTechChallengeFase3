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