#[derive(Debug)]
pub enum PaymentStatus {
    Pending = 1,
    Paid = 2,
    Cancelled = 4
}