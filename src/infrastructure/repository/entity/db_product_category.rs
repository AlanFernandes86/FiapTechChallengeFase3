use crate::domain::entities::product_category::ProductCategory;

#[derive(sqlx::FromRow)]
pub struct DbProductCategory {
    pub id: i32,
    pub name: String,
    pub description: String
}

// Implementando o From trait para o Client
impl From<DbProductCategory> for ProductCategory {
    fn from(tb_product_category: DbProductCategory) -> Self {
        ProductCategory {
            id: tb_product_category.id,
            name: tb_product_category.name,
            description: tb_product_category.description
        }
    }
}