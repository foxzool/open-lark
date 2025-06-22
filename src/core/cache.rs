use std::time::Duration;

use quick_cache::sync::Cache;
use tokio::time::Instant;

#[derive(Debug)]
pub struct QuickCache<T: Clone> {
    cache: Cache<String, (T, Instant)>,
}

impl<T: Clone> Default for QuickCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> QuickCache<T> {
    pub fn new() -> Self {
        let cache = Cache::new(10);
        Self { cache } //
    }

    pub fn get(&self, key: &str) -> Option<T> {
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

    /// Set a key-value pair with an expire time in seconds.
    pub fn set(&mut self, key: &str, value: T, expire_time: i32) {
        let expire_time = Instant::now() + Duration::from_secs(expire_time as u64);
        self.cache.insert(key.to_string(), (value, expire_time));
    }
}
