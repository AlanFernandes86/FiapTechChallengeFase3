use crate::domain::entities::{order_product::OrderProduct, product_category::ProductCategory};

#[derive(sqlx::FromRow)]
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

impl From<DbOrderProduct> for OrderProduct {
    fn from(db_order_product: DbOrderProduct) -> Self {
        OrderProduct {
            order_product_id: db_order_product.order_product_id,
            order_id: db_order_product.order_id,
            product_id: db_order_product.product_id,
            name: db_order_product.name,
            price: db_order_product.price,
            quantity: db_order_product.quantity,
            description: db_order_product.description,
            image_url: db_order_product.image_url,
            product_category: ProductCategory {
                id: db_order_product.product_category_id,
                name: db_order_product.product_category_name,
                description: db_order_product.product_category_description
            },
            // updated_at: NaiveDateTime::from_timestamp(0, 0),
            // created_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }
}