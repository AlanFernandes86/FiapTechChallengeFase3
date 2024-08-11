use async_trait::async_trait;
use sqlx::mssql::MssqlPool;
use crate::domain::entities::client::Client;
use crate::domain::repositories::client_repository::ClientRepository;

pub struct SqlClientRepository {
    // pool: MssqlPool,
}

impl SqlClientRepository {
    // O método new é definido diretamente na struct, não na trait
    pub fn new() -> Self {
        SqlClientRepository {  }
    }
}

#[async_trait]
impl ClientRepository for SqlClientRepository {
      async fn get_client_by_cpf(&self, cpf: String) -> Option<Client> {
        // Retorna um cliente de exemplo
        Some(
        Client {
            cpf: cpf,
            name: "John Doe".to_string(),
            email: "adad".to_string()
        })
    }
}