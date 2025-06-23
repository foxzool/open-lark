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

    /// 获取值和过期信息
    pub fn get_with_expiry(&self, key: &str) -> Option<CacheEntry<T>> {
        match self.cache.get(key) {
            None => None,
            Some((value, expire_time)) => {
                let now = Instant::now();
                if expire_time > now {
                    Some(CacheEntry {
                        value,
                        expires_at: expire_time,
                        current_time: now,
                    })
                } else {
                    self.cache.remove(key);
                    None
                }
            }
        }
    }
}

/// 缓存条目信息，包含值和过期时间
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub expires_at: Instant,
    pub current_time: Instant,
}

impl<T> CacheEntry<T> {
    /// 获取剩余的过期秒数
    pub fn expiry_seconds(&self) -> u64 {
        if self.expires_at > self.current_time {
            (self.expires_at - self.current_time).as_secs()
        } else {
            0
        }
    }

    /// 检查是否即将过期（剩余时间少于指定秒数）
    pub fn expires_within(&self, seconds: u64) -> bool {
        self.expiry_seconds() <= seconds
    }
}
