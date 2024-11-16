use std::error::Error;
use crate::domain::{ constants::user_group::UserGroup, entities::user::User, repository::user_repository::UserRepository };

pub struct SetUserUseCase {
    client_repository: Box<dyn UserRepository>,
}

impl SetUserUseCase {
    pub fn new(client_repository: Box<dyn UserRepository>) -> Self {
        SetUserUseCase {
            client_repository
        }
    }

    pub async fn handle(&self, user: User) -> Result<(), Box<dyn Error>> {
        if UserGroup::validate(&user.group) == false {
            return Err("Invalid user group".into());
        }

        self.client_repository.set_user(user).await
    }
}