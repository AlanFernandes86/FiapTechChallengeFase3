#[derive(serde::Serialize, Debug)]
pub struct Product {
    pub id: i32,
    pub product_category_id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String
}