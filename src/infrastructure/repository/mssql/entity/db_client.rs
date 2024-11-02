use crate::domain::entities::client::Client;

#[derive(sqlx::FromRow)]
pub struct DbClient {
	pub cpf: String,
	pub name: String,
	pub email: String
}

// Implementando o From trait para o Client
impl From<DbClient> for Client {
    fn from(tb_client: DbClient) -> Self {
        Client {
            cpf: tb_client.cpf,
            name: tb_client.name,
            email: tb_client.email,
        }
    }
}