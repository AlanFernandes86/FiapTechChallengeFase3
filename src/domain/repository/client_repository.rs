use std::error::Error;
use async_trait::async_trait;
use crate::domain::entities::client::Client;

#[async_trait]
pub trait ClientRepository {
	async fn get_client_by_cpf(&self, cfp: String) -> Result<Option<Client>, Box<dyn Error>>;
	async fn set_client(&self, client: Client) -> Result<(), Box<dyn Error>>;
}