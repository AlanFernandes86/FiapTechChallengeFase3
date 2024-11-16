#[derive(serde::Serialize, Debug)]
pub struct User {
	pub cpf: String,
	pub name: String,
	pub email: String,
	pub group: String
}