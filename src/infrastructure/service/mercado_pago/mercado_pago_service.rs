use std::{env, error::Error};
use async_trait::async_trait;
use crate::domain::{
    entities::order::Order, 
    service::{models::start_payment_response::StartPaymentResponse, payment_service::PaymentService}
};

use super::{create_order_request::{CreateMpOrderRequest, CreateMpOrderRequestItem}, get_pos_list_response::GetPosListResponse, get_token_request::GetTokenRequest};

pub struct MercadoPagoService {
    http_client: reqwest::Client,
    base_url: String,
    notification_url: String,
    client_id: String,
    client_secret: String,
    user_id: String,
    external_store_id: String
}

impl MercadoPagoService {

    // O método new é definido diretamente na struct
    pub fn new(http_client: reqwest::Client) -> Self {
        let base_url = env::var("MERCADO_PAGO_URL").expect("MERCADO_PAGO_URL not found in .env file");
        let notification_url = env::var("MERCADO_PAGO_NOTIFICATION_URL").expect("MERCADO_PAGO_NOTIFICATION_URL not found in .env file");
        let client_id = env::var("MERCADO_PAGO_CLIENT_ID").expect("MERCADO_PAGO_CLIENT_ID not found in .env file");
        let client_secret = env::var("MERCADO_PAGO_CLIENT_SECRET").expect("MERCADO_PAGO_CLIENT_SECRET not found in .env file");
        let user_id = env::var("MERCADO_PAGO_USER_ID").expect("MERCADO_PAGO_USER_ID not found in .env file");
        let external_store_id = env::var("MERCADO_PAGO_EXTERNAL_STORE_ID").expect("MERCADO_PAGO_EXTERNAL_STORE_ID not found in .env file");
        MercadoPagoService { 
            http_client,
            base_url,
            notification_url,
            client_id,
            client_secret,
            user_id,
            external_store_id
        }
    }

    pub async fn get_access_token(&self) -> Result<String, Box<dyn Error>> {
        let request_uri = self.base_url.clone() + "/oauth/token";
        let request_payload = GetTokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            test_token: true
        };

        let response = self.http_client.post(request_uri)
            .form(&[("grant_type", "client_credentials"), ("client_id", "client_id"), ("client_secret", "client_secret")])
            .json(&request_payload)
            .send()
            .await?;        

        let status = response.status();
        let response_body: serde_json::Value = response.json().await?;
        match status {
            reqwest::StatusCode::OK => {
                let token = response_body["access_token"].as_str().unwrap();
                Ok(token.to_string())
            },
            _ => {
                println!("{:?}", response_body);
                Err(response_body["error"].as_str().unwrap().into())
            }
        }
    }

    pub async fn create_in_person_order_by_qrcode(&self, order: &Order, pos_external_id: &str, mp_token: &str) -> Result<(), Box<dyn Error>> {
        let request_payload = CreateMpOrderRequest{
            external_reference: pos_external_id.to_string() + "#" + order.id.to_string().as_str(),
            title: "ChallengeFastFood".to_string(),
            description: "Pedido do ChallengeFastFood".to_string(),
            notification_url: self.notification_url.clone(),
            total_amount: order.total,
            items: vec![
                CreateMpOrderRequestItem {
                    title: "[ChallengeFastFood] - Pedido nº ".to_string() + &order.id.to_string(), 
                    description: "Este é o pagamento do pedido id: ".to_string() + &order.id.to_string(),
                    quantity: 1,
                    unit_price: order.total,
                    unit_measure: "pack".to_string(),
                    total_amount: order.total
                }
            ],
        };

        let request_uri = self.base_url.clone() + "/instore/qr/seller/collectors/" + &self.user_id.clone() + "/stores/" + &self.external_store_id + "/pos/" + pos_external_id + "/orders";
        let response = self.http_client.put(request_uri)
            .header("Authorization", "Bearer ".to_string() + &mp_token)
            .json(&request_payload)
            .send()
            .await?;
        
        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            _ => Err(Box::new(response.error_for_status().unwrap_err()))            
        }
    }

    pub async fn get_pos_list(&self, mp_token: &str) -> Result<GetPosListResponse, Box<dyn Error>> {
        let request_uri = self.base_url.clone() + "/pos";
        let response = self.http_client.get(request_uri)
            .header("Authorization", "Bearer ".to_string() + &mp_token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let response_body: GetPosListResponse = response.json().await?;
                Ok(response_body)
            },
            _ => Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn get_resource<T>(&self, uri: &str, mp_token: &str) -> Result<T, Box<dyn Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.http_client.get(uri)
            .header("Authorization", "Bearer ".to_string() + &mp_token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let response_body: T = response.json().await?;
                Ok(response_body)
            },
            _ => Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }
}

#[async_trait]
impl PaymentService for MercadoPagoService {
    async fn start_payment(&self, order: &Order, pdv_id: String) -> Result<StartPaymentResponse, Box<dyn Error>> {
        let mp_token = self.get_access_token().await?;
        let pos_list = self.get_pos_list(&mp_token).await?;
        let pos = pos_list.results.iter().find(|pos| pos.external_id == pdv_id).ok_or(format!("pdv_id {0} not found", pdv_id))?;
        let result = self.create_in_person_order_by_qrcode(order, &pos.external_id, &mp_token).await;

        match result {
            Ok(_) => {
                let start_payment_response = StartPaymentResponse {
                    qr_url_image: Some(pos.qr.image.clone())
                };
                Ok(start_payment_response)
            },
            Err(e) => return Err(e)
        }
    }
}