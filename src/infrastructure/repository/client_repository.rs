use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::client::Client;
use crate::domain::repository::client_repository::ClientRepository;
use crate::infrastructure::repository::entity::db_client::DbClient;

pub struct MssqlClientRepository {
    pool: Arc<MssqlPool>,
}

impl MssqlClientRepository {
    // O método new é definido diretamente na struct
    pub fn new(pool: Arc<MssqlPool>) -> Self {
        MssqlClientRepository { pool }
    }
}

#[async_trait]
impl ClientRepository for MssqlClientRepository {
    async fn get_client_by_cpf(&self, cpf: String) -> Result<Option<Client>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbClient>(
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

    async fn set_client(&self, client: Client) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            "INSERT INTO TechChallenge.dbo.client (cpf, name, email, updated_at, created_at) VALUES (@p1, @p2, @p3, GETDATE(), GETDATE())"
        )
        .bind(client.cpf)
        .bind(client.name)
        .bind(client.email)
        .execute(&*self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}