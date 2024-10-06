use crate::domain::entities::{order_product::OrderProduct, product_category::ProductCategory};

#[derive(serde::Deserialize, Debug)]
pub struct PutOrderProductDTO {
    pub order_product_id: Option<i32>,
    pub order_id: i32,
    pub product_id: i32,
    pub price: f64,
    pub quantity: i32
}

impl From<PutOrderProductDTO> for OrderProduct {
    fn from(dto: PutOrderProductDTO) -> Self {
        OrderProduct {
            order_product_id: dto.order_product_id,
            order_id: dto.order_id,
            product_id: dto.product_id,
            price: dto.price,
            quantity: dto.quantity,
            name: "".to_string(),
            description: "".to_string(),
            image_url: "".to_string(),
            product_category: ProductCategory {
                id: 0,
                name: "".to_string(),
                description: "".to_string()
            }
        }
    }
}