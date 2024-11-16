use std::error::Error;
use crate::domain::{ entities::user::User, repository::user_repository::UserRepository };

pub struct SetUserUseCase {
    client_repository: Box<dyn UserRepository>,
}

impl SetUserUseCase {
    pub fn new(client_repository: Box<dyn UserRepository>) -> Self {
        SetUserUseCase {
            client_repository
        }
    }

    pub async fn handle(&self, client: User) -> Result<(), Box<dyn Error>> {
        self.client_repository.set_user(client).await
    }
}