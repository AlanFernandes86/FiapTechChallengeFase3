use crate::domain::entities::user::User;

#[derive(sqlx::FromRow)]
pub struct DbUser {
	pub cpf: String,
	pub name: String,
	pub email: String
}

// Implementando o From trait para o Client
impl From<DbUser> for User {
    fn from(tb_client: DbUser) -> Self {
        User {
            cpf: tb_client.cpf,
            name: tb_client.name,
            email: tb_client.email,
            group: "".to_string()
        }
    }
}