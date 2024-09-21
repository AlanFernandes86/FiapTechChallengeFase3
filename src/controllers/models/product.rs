use serde::Deserialize;
use crate::domain::entities::product_category::ProductCategory;

#[derive(serde::Deserialize, Debug)]
pub struct ProductDTO {
    pub id: Option<i32>,
    pub category_id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String
}

#[derive(Deserialize)]
pub struct ProductQuery {
    pub category_id: i32,
}

impl From<ProductDTO> for crate::domain::entities::product::Product {
    fn from(product_dto: ProductDTO) -> Self {
        Self {
            id: product_dto.id,
            product_category: ProductCategory {
                id: product_dto.category_id,
                name: "".to_string(),
                description: "".to_string()
            },
            name: product_dto.name,
            description: product_dto.description,
            price: product_dto.price,
            image_url: product_dto.image_url
        }
    }
}