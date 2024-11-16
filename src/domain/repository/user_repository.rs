use std::error::Error;
use async_trait::async_trait;
use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository {
	async fn get_user_by_cpf(&self, cfp: String) -> Result<Option<User>, Box<dyn Error>>;
	async fn set_user(&self, client: User) -> Result<(), Box<dyn Error>>;
}