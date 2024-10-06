use std::error::Error;
use crate::domain::entities::client::Client;
use crate::domain::repository::client_repository::ClientRepository;

pub struct GetClientByCpfUseCase {
    client_repository: Box<dyn ClientRepository>,
}

impl GetClientByCpfUseCase {
    pub fn new(client_repository: Box<dyn ClientRepository>) -> Self {
        GetClientByCpfUseCase {
            client_repository
        }
    }

    pub async fn handle(&self, cpf: String) -> Result<Option<Client>, Box<dyn Error>> {
        self.client_repository.get_client_by_cpf(cpf).await
    }
}