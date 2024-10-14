use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenRequest {
    pub client_secret: String,
    pub client_id: String,
    pub grant_type: String,
    pub test_token: bool
}