#[derive(serde::Serialize, Debug)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: String
}