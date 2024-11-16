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
        let cpf = item.get("cpf")
        .and_then(|v| v.as_s().ok())
        .map(String::from)
        .unwrap_or_default();
    
        let name = item.get("name")
        .and_then(|v| v.as_s().ok())
        .map(String::from)
        .unwrap_or_default();

        let email = item.get("email")
        .and_then(|v| v.as_s().ok())
        .map(String::from)
        .unwrap_or_default();

        let group = item.get("group")
        .and_then(|v| v.as_s().ok())
        .map(String::from)
        .unwrap_or_default();

        DbUser { cpf, name, email, group }
    }

    pub fn into(self) -> User {
        User { cpf: self.cpf, name: self.name, email: self.email, group: self.group }
    }
}
