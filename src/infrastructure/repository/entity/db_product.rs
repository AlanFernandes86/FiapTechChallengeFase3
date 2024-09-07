use crate::domain::entities::product::Product;

#[derive(sqlx::FromRow)]
pub struct DbProduct {
    pub id: i32,
    pub product_category_id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String
}

impl From<DbProduct> for Product {
    fn from(db_product: DbProduct) -> Self {
        Product {
            id: db_product.id,
            product_category_id: db_product.product_category_id,
            name: db_product.name,
            description: db_product.description,
            price: db_product.price,
            image_url: db_product.image_url
        }
    }
}