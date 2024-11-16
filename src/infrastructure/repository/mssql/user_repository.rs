use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::user::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::infrastructure::repository::mssql::entity::db_user::DbUser;

pub struct MssqlUserRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlUserRepository {
    // O método new é definido diretamente na struct
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for MssqlUserRepository {
    async fn get_user_by_cpf(&self, cpf: String) -> Result<Option<User>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbUser>(
            "SELECT cpf, name, email FROM TechChallenge.dbo.client WHERE cpf = @p1"
        )
        .bind(cpf)
        .fetch_optional(&*self.pool) 
        .await;
        
        match result {
            Ok(option) => {
                match option {
                    Some(client) => Ok(Some(client.into())),
                    None => Ok(None)
                }
            },
            Err(e) => Err(Box::new(e))
        }  
    }

    async fn set_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            "INSERT INTO TechChallenge.dbo.client (cpf, name, email, updated_at, created_at) VALUES (@p1, @p2, @p3, GETDATE(), GETDATE())"
        )
        .bind(user.cpf)
        .bind(user.name)
        .bind(user.email)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}