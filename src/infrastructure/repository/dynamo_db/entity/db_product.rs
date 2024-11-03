use crate::domain::entities::product::Product;
use aws_sdk_dynamodb::types::AttributeValue;
use crate::domain::entities::product_category::ProductCategory;

pub struct DbProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String,
    pub available: bool
}

impl DbProduct {
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

        let price = item.get("price")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or_default();

        let image_url = item.get("image_url")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let available = item.get("available")
            .and_then(|v| v.as_bool().ok())
            .copied()
            .unwrap_or_default();

        Self { id, name, description, price, image_url, available }
    }

    pub fn into_domain(self) -> Product {
        Product {
            id: Some(self.id),
            name: self.name,
            description: self.description,
            price: self.price,
            image_url: self.image_url,
            available: self.available,
            product_category: ProductCategory::default()
        }
    }
}