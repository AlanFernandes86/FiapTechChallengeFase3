use cached::UnboundCache;
use std::sync::Mutex;

struct MemoryCache {
    cache: Mutex<UnboundCache<String, String>>,
}

impl MemoryCache {
    fn new() -> Self {
        Self {
            cache: Mutex::new(UnboundCache::new()),
        }
    }

    fn set(&self, key: String, value: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.cache_set(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.cache_get(key).cloned()
    }
}