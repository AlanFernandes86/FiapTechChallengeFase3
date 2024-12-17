use crate::domain::entities::{product::Product, product_category::ProductCategory};

#[derive(sqlx::FromRow)]
pub struct DbProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String,
    pub product_category_id: i32,
    pub product_category_name: String,
    pub product_category_description: String
}

impl From<DbProduct> for Product {
    fn from(db_product: DbProduct) -> Self {
        Product {
            id: Some(db_product.id),
            name: db_product.name,
            description: db_product.description,
            price: db_product.price,
            image_url: db_product.image_url,
            available: true,
            product_category: ProductCategory {
                id: db_product.product_category_id,
                name: db_product.product_category_name,
                description: db_product.product_category_description            
            }
        }
    }
}