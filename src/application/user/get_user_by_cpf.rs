use std::error::Error;
use crate::domain::entities::user::User;
use crate::domain::repository::user_repository::UserRepository;

pub struct GetUserByCpfUseCase {
    user_repository: Box<dyn UserRepository>,
}

impl GetUserByCpfUseCase {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        GetUserByCpfUseCase {
            user_repository
        }
    }

    pub async fn handle(&self, cpf: String) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository.get_user_by_cpf(cpf).await
    }
}