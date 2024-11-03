#[derive(serde::Serialize, Debug)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl ProductCategory {
    pub fn default() -> Self {
        ProductCategory {
            id: 0,
            name: "".to_string(),
            description: "".to_string()
        }
    }

    pub fn clone(&self) -> Self {
        ProductCategory {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone()
        }
    }
}