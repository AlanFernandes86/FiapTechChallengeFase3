use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartPaymentResponse {
    pub qr_url_image: Option<String>,
}