use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;
use crate::domain::entities::client::Client;

pub struct DbClient {
	pub cpf: String,
	pub name: String,
	pub email: String
}

impl DbClient {
    pub fn from_item(item: HashMap<String, AttributeValue>) -> Self {
        let cpf = item.get("cpf").unwrap().as_s().unwrap().to_string();
        let name = item.get("name").unwrap().as_s().unwrap().to_string();
        let email = item.get("email").unwrap().as_s().unwrap().to_string();
        DbClient { cpf, name, email }
    }

    pub fn into(self) -> Client {
        Client { cpf: self.cpf, name: self.name, email: self.email }
    }
}
