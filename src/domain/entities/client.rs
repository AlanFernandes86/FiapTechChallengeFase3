#[derive(serde::Serialize)]
pub struct Client {
	pub cpf: String,
	pub name: String,
	pub email: String
}