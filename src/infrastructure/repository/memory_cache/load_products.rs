use std::error::Error;
use crate::infrastructure::repository::memory_cache::memory_cache::MemoryCache;
use crate::{domain::repository::product_repository::ProductRepository, infrastructure::repository::dynamo_db::{common::dynamo_db_factory::DynamoDbFactory, product_repository::DynamoDbProductRepository}};

pub struct LoadProducts<'a> {
  cache: &'a MemoryCache,
}

impl<'a> LoadProducts<'a> {
  pub fn new() -> Self {
    LoadProducts {
      cache: MemoryCache::instance(),
    }
  }

  pub async fn load_products(&self) -> Result<(), Box<dyn Error>> {
    let factory = DynamoDbFactory::get_instance().await;
    
    let client = match factory {
      Ok(c) => c,
      Err(e) => return Err(Box::new(e)),
    };

    let product_repository = DynamoDbProductRepository::new(client.clone());

    let result = product_repository.get_products().await;

    match result {
      Ok(products) => {
        if let Some(products) = products {
          self.cache.set("products".to_string(), products);
        }
        Ok(())
      },
      Err(e) => return Err(e),
    }
  }
}