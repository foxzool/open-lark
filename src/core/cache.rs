use std::collections::HashMap;
use std::time::Duration;

pub trait Cache {
    fn new() -> Self;
    fn set(&mut self, key: &str, value: &str, expire_time: Duration);
    fn get(&self, key: &str) -> Option<String>;
}

pub struct LocalCache {
    cache: HashMap<String, (String, Duration)>,
}

impl Cache for LocalCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: &str, expire_time: Duration) {
        self.cache
            .insert(key.to_string(), (value.to_string(), expire_time));
    }

    fn get(&self, key: &str) -> Option<String> {
        if let Some((value, expire_time)) = self.cache.get(key) {
            if expire_time.as_secs() > 0 {
                return Some(value.to_string());
            } else {
                None
            }
        } else {
            None
        }
    }
}
