use cached::{Cached, UnboundCache};
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
use crate::domain::entities::product::Product;

pub struct MemoryCache {
    products_cache: Mutex<UnboundCache<String, HashMap<i32, Product>>>,
}

impl MemoryCache {
    fn new() -> Self {
        Self {
            products_cache: Mutex::new(UnboundCache::new()),
        }
    }

    pub fn set(&self, key: String, value: HashMap<i32, Product>) {
        let mut cache = self.products_cache.lock().unwrap();
        cache.cache_set(key, value);
    }

    pub fn get(&self, key: String) -> Option<HashMap<i32, Product>> {
        let mut cache = self.products_cache.lock().unwrap();
        cache.cache_get(&key).cloned()
    }
}

static INSTANCE: Lazy<MemoryCache> = Lazy::new(|| MemoryCache::new());

impl MemoryCache {
    pub fn instance() -> &'static MemoryCache {
        &INSTANCE
    }
}