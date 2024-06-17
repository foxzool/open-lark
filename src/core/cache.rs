use std::time::Duration;

use quick_cache::sync::Cache;
use tokio::time::Instant;

pub struct QuickCache {
    cache: Cache<String, (String, Instant)>,
}

impl Default for QuickCache {
    fn default() -> Self {
        Self::new()
    }
}

impl QuickCache {
    pub fn new() -> Self {
        let cache = Cache::new(10);
        Self { cache } //
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.cache.get(key) {
            None => None,
            Some((value, expire_time)) => {
                if expire_time > Instant::now() {
                    Some(value)
                } else {
                    self.cache.remove(key);
                    None
                }
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str, expire_time: i32) {
        let expire_time = Instant::now() + Duration::from_secs(expire_time as u64);
        self.cache
            .insert(key.to_string(), (value.to_string(), expire_time));
    }
}
