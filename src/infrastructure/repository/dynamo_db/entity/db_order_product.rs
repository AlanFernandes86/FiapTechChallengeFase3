use aws_sdk_dynamodb::{operation::import_table, types::AttributeValue};

use crate::domain::entities::{order_product::OrderProduct, product_category::ProductCategory};

pub struct DbOrderProduct {
    pub order_product_id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub description: String,
    pub image_url: String,
    pub product_category_id: i32,
    pub product_category_name: String,
    pub product_category_description: String,
    // pub updated_at: NaiveDateTime,
    // pub created_at: NaiveDateTime,
}

impl DbOrderProduct {
    pub fn from_item(item: std::collections::HashMap<String, AttributeValue>) -> Self {
        let order_product_id = item.get("order_product_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let order_id = item.get("order_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let product_id = item.get("product_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let name = item.get("name")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let quantity = item.get("quantity")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let price = item.get("price")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or_default();

        let description = item.get("description")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let image_url = item.get("image_url")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let product_category_id = item.get("product_category_id")
            .and_then(|v| v.as_n().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or_default();

        let product_category_name = item.get("product_category_name")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        let product_category_description = item.get("product_category_description")
            .and_then(|v| v.as_s().ok())
            .map(String::from)
            .unwrap_or_default();

        Self { order_product_id, order_id, product_id, name, quantity, price, description, image_url, product_category_id, product_category_name, product_category_description }
    }
}

impl From<DbOrderProduct> for OrderProduct {
    fn from(order_product: DbOrderProduct) -> Self {
        Self {
            order_product_id: Some(order_product.order_product_id),
            order_id: order_product.order_id,
            product_id: order_product.product_id,
            name: order_product.name,
            quantity: order_product.quantity,
            price: order_product.price,
            description: order_product.description,
            image_url: order_product.image_url,
            product_category: ProductCategory {
                id: order_product.product_category_id,
                name: order_product.product_category_name,
                description: order_product.product_category_description,
            }
        }
    }
}