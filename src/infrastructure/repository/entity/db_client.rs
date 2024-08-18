#[derive(sqlx::FromRow)]
pub struct DbClient {
	pub cpf: String,
	pub name: String,
	pub email: String
}