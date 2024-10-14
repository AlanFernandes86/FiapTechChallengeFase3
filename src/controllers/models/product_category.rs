use crate::domain::entities::product_category::ProductCategory;

#[derive(serde::Deserialize, Debug)]
pub struct ProductCategoryDTO {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl From<ProductCategoryDTO> for ProductCategory {
    fn from(dto: ProductCategoryDTO) -> Self {
        ProductCategory {
            id: dto.id,
            name: dto.name,
            description: dto.description
        }
    }
}
