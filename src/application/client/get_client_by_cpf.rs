use crate::domain::entities::client::Client;
use crate::domain::repositories::client_repository::ClientRepository;

pub struct GetClientByCpfUseCase {
    client_repository: Box<dyn ClientRepository>,
}

impl GetClientByCpfUseCase {
    pub fn new(client_repository: Box<dyn ClientRepository>) -> Self {
        GetClientByCpfUseCase {
            client_repository
        }
    }

    pub async fn handle(&self, cpf: String) -> Option<Client> {
        self.client_repository.get_client_by_cpf(cpf).await
    }
}