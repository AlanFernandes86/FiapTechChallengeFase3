use aws_sdk_dynamodb::types::AttributeValue;
use crate::domain::entities::product_category::ProductCategory;

pub struct DbProductCategory {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl DbProductCategory {
    pub fn from_item(item: std::collections::HashMap<String, AttributeValue>) -> Self {
        let id = item.get("id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let name = item.get("name")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let description = item.get("description")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default(); 

        Self { id, name, description }        
    }

    pub fn into_domain(self) -> ProductCategory {
        ProductCategory {
            id: self.id,
            name: self.name,
            description: self.description
        }
    }
}