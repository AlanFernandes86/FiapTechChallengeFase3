#[derive(serde::Serialize, Debug)]
pub struct Client {
	pub cpf: String,
	pub name: String,
	pub email: String
}