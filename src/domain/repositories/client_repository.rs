use async_trait::async_trait;
use crate::domain::entities::client::Client;

#[async_trait]
pub trait ClientRepository {
	async fn get_client_by_cpf(&self, cfp: String) -> Option<Client>;
}