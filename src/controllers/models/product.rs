use serde::Deserialize;

#[derive(serde::Deserialize, Debug)]
pub struct ProductDTO {
    pub id: i32,
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