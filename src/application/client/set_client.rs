use std::error::Error;
use crate::domain::{ entities::client::Client, repository::client_repository::ClientRepository };

pub struct SetClientUseCase {
    client_repository: Box<dyn ClientRepository>,
}

impl SetClientUseCase {
    pub fn new(client_repository: Box<dyn ClientRepository>) -> Self {
        SetClientUseCase {
            client_repository
        }
    }

    pub async fn handle(&self, client: Client) -> Result<(), Box<dyn Error>> {
        self.client_repository.set_client(client).await
    }
}