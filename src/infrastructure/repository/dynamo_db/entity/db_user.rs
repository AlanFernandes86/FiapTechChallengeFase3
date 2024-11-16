use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;
use crate::domain::entities::user::User;

pub struct DbUser {
	pub cpf: String,
	pub name: String,
	pub email: String,
    pub group: String
}

impl DbUser {
    pub fn from_item(item: HashMap<String, AttributeValue>) -> Self {
        let cpf = item.get("cpf").unwrap().as_s().unwrap().to_string();
        let name = item.get("name").unwrap().as_s().unwrap().to_string();
        let email = item.get("email").unwrap().as_s().unwrap().to_string();
        let group = item.get("group").unwrap().as_s().unwrap().to_string();
        DbUser { cpf, name, email, group }
    }

    pub fn into(self) -> User {
        User { cpf: self.cpf, name: self.name, email: self.email, group: self.group }
    }
}
