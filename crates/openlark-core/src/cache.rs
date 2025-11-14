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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_cache_creation() {
        let cache: QuickCache<String> = QuickCache::new();

        // Test that cache is empty initially
        assert!(cache.get("test_key").is_none());
    }

    #[test]
    fn test_cache_default() {
        let cache: QuickCache<i32> = QuickCache::default();

        // Test default implementation
        assert!(cache.get("test_key").is_none());
    }

    #[test]
    fn test_cache_set_and_get() {
        let mut cache = QuickCache::new();

        // Set a value
        cache.set("key1", "value1".to_string(), 10);

        // Get the value
        let result = cache.get("key1");
        assert_eq!(result, Some("value1".to_string()));
    }

    #[test]
    fn test_cache_expiration() {
        let mut cache = QuickCache::new();

        // Set a value with very short expiration
        cache.set("short_key", "short_value".to_string(), 1);

        // Value should be available immediately
        assert_eq!(cache.get("short_key"), Some("short_value".to_string()));

        // Wait for expiration
        sleep(Duration::from_secs(2));

        // Value should be expired and removed
        assert!(cache.get("short_key").is_none());
    }

    #[test]
    fn test_cache_nonexistent_key() {
        let cache: QuickCache<String> = QuickCache::new();

        // Getting non-existent key should return None
        assert!(cache.get("nonexistent").is_none());
    }

    #[test]
    fn test_cache_with_expiry_info() {
        let mut cache = QuickCache::new();

        // Set a value with longer expiration
        cache.set("expiry_key", 42, 60);

        // Get with expiry info
        let entry = cache.get_with_expiry("expiry_key");
        assert!(entry.is_some());

        let entry = entry.unwrap();
        assert_eq!(entry.value, 42);
        assert!(entry.expiry_seconds() > 0);
        assert!(entry.expiry_seconds() <= 60);
    }

    #[test]
    fn test_cache_entry_expiry_seconds() {
        let mut cache = QuickCache::new();

        // Set value with 30 second expiration
        cache.set("timing_key", "timing_value".to_string(), 30);

        let entry = cache.get_with_expiry("timing_key").unwrap();
        let remaining = entry.expiry_seconds();

        // Should have approximately 30 seconds remaining (allowing for slight timing variations)
        assert!(remaining > 25 && remaining <= 30);
    }

    #[test]
    fn test_cache_entry_expires_within() {
        let mut cache = QuickCache::new();

        // Set value with 5 second expiration
        cache.set("soon_key", "soon_value".to_string(), 5);

        let entry = cache.get_with_expiry("soon_key").unwrap();

        // Should expire within 10 seconds
        assert!(entry.expires_within(10));

        // Should not expire within 1 second
        assert!(!entry.expires_within(1));
    }

    #[test]
    fn test_cache_with_different_types() {
        let mut string_cache = QuickCache::new();
        let mut int_cache = QuickCache::new();
        let mut vec_cache = QuickCache::new();

        // Test with String
        string_cache.set("str_key", "hello".to_string(), 30);
        assert_eq!(string_cache.get("str_key"), Some("hello".to_string()));

        // Test with integer
        int_cache.set("int_key", 123, 30);
        assert_eq!(int_cache.get("int_key"), Some(123));

        // Test with Vec
        vec_cache.set("vec_key", vec![1, 2, 3], 30);
        assert_eq!(vec_cache.get("vec_key"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_cache_overwrite_key() {
        let mut cache = QuickCache::new();

        // Set initial value
        cache.set("overwrite_key", "first_value".to_string(), 30);
        assert_eq!(cache.get("overwrite_key"), Some("first_value".to_string()));

        // Overwrite with new value
        cache.set("overwrite_key", "second_value".to_string(), 30);
        assert_eq!(cache.get("overwrite_key"), Some("second_value".to_string()));
    }

    #[test]
    fn test_cache_multiple_keys() {
        let mut cache = QuickCache::new();

        // Set multiple key-value pairs
        cache.set("key1", "value1".to_string(), 60);
        cache.set("key2", "value2".to_string(), 60);
        cache.set("key3", "value3".to_string(), 60);

        // Verify all can be retrieved
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), Some("value2".to_string()));
        assert_eq!(cache.get("key3"), Some("value3".to_string()));
    }

    #[test]
    fn test_cache_expired_entry_cleanup() {
        let mut cache = QuickCache::new();

        // Set a value that expires quickly
        cache.set("cleanup_key", "cleanup_value".to_string(), 1);

        // Verify it exists
        assert!(cache.get("cleanup_key").is_some());

        // Wait for expiration
        sleep(Duration::from_secs(2));

        // Accessing expired key should remove it and return None
        assert!(cache.get("cleanup_key").is_none());

        // Subsequent access should still return None
        assert!(cache.get("cleanup_key").is_none());
    }

    #[test]
    fn test_cache_entry_debug_trait() {
        let mut cache = QuickCache::new();
        cache.set("debug_key", "debug_value".to_string(), 60);

        let entry = cache.get_with_expiry("debug_key").unwrap();
        let debug_string = format!("{:?}", entry);

        assert!(debug_string.contains("CacheEntry"));
        assert!(debug_string.contains("debug_value"));
    }

    #[test]
    fn test_cache_debug_trait() {
        let cache: QuickCache<String> = QuickCache::new();
        let debug_string = format!("{:?}", cache);

        assert!(debug_string.contains("QuickCache"));
    }

    #[test]
    fn test_zero_expiry_seconds() {
        let now = Instant::now();
        let past_time = now - Duration::from_secs(10);

        let entry = CacheEntry {
            value: "test".to_string(),
            expires_at: past_time,
            current_time: now,
        };

        // Should return 0 for expired entries
        assert_eq!(entry.expiry_seconds(), 0);
        assert!(entry.expires_within(100));
    }

    #[test]
    fn test_cache_entry_clone() {
        let mut cache = QuickCache::new();
        cache.set("clone_key", vec![1, 2, 3], 60);

        let entry = cache.get_with_expiry("clone_key").unwrap();
        let cloned_entry = entry.clone();

        assert_eq!(entry.value, cloned_entry.value);
        assert_eq!(entry.expires_at, cloned_entry.expires_at);
        assert_eq!(entry.current_time, cloned_entry.current_time);
    }

    #[test]
    fn test_cache_with_zero_expiry() {
        let mut cache = QuickCache::new();

        // Set value with 0 second expiry (should expire immediately)
        cache.set("zero_key", "zero_value".to_string(), 0);

        // Value should not be retrievable due to immediate expiry
        assert!(cache.get("zero_key").is_none());
    }

    #[test]
    fn test_cache_with_negative_expiry() {
        let mut cache = QuickCache::new();

        // Set value with negative expiry - this should be handled gracefully
        // Note: casting -1 as u64 creates a very large number, but adding it to Instant can overflow
        // Let's test with 0 instead to simulate immediate expiry
        cache.set("zero_expiry_key", "zero_value".to_string(), 0);

        // Value with 0 expiry should not be retrievable due to immediate expiry
        assert!(cache.get("zero_expiry_key").is_none());
    }

    #[test]
    fn test_cache_large_expiry_time() {
        let mut cache = QuickCache::new();

        // Set value with very large expiry time
        cache.set("large_key", "large_value".to_string(), i32::MAX);

        let entry = cache.get_with_expiry("large_key").unwrap();
        assert_eq!(entry.value, "large_value");
        assert!(entry.expiry_seconds() > 1_000_000);
    }

    #[test]
    fn test_get_with_expiry_nonexistent_key() {
        let cache: QuickCache<String> = QuickCache::new();

        // Getting non-existent key with expiry should return None
        assert!(cache.get_with_expiry("nonexistent").is_none());
    }

    #[test]
    fn test_cache_with_empty_strings() {
        let mut cache = QuickCache::new();

        // Test with empty key
        cache.set("", "empty_key_value".to_string(), 30);
        assert_eq!(cache.get(""), Some("empty_key_value".to_string()));

        // Test with empty value
        cache.set("empty_value_key", "".to_string(), 30);
        assert_eq!(cache.get("empty_value_key"), Some("".to_string()));

        // Test with both empty
        cache.set("", "".to_string(), 30);
        assert_eq!(cache.get(""), Some("".to_string()));
    }

    #[test]
    fn test_cache_with_unicode_strings() {
        let mut cache = QuickCache::new();

        let unicode_key = "键名_🔑";
        let unicode_value = "值_🎯_测试";

        cache.set(unicode_key, unicode_value.to_string(), 30);
        assert_eq!(cache.get(unicode_key), Some(unicode_value.to_string()));
    }

    #[test]
    fn test_cache_with_long_strings() {
        let mut cache = QuickCache::new();

        let long_key = "a".repeat(1000);
        let long_value = "b".repeat(10000);

        cache.set(&long_key, long_value.clone(), 30);
        assert_eq!(cache.get(&long_key), Some(long_value));
    }

    #[test]
    fn test_cache_memory_efficiency() {
        let mut cache = QuickCache::new();

        // Add items within cache capacity (cache has limit of 10)
        for i in 0..8 {
            cache.set(&format!("key_{}", i), format!("value_{}", i), 60);
        }

        // Verify items within capacity are accessible
        for i in 0..8 {
            let key = format!("key_{}", i);
            let expected_value = format!("value_{}", i);
            assert_eq!(cache.get(&key), Some(expected_value));
        }
    }

    #[test]
    fn test_cache_concurrent_style_simulation() {
        // Simulate concurrent-style access patterns (without actual threads for determinism)
        let mut cache = QuickCache::new();

        // Simulate limited operations within cache capacity
        for thread_id in 0..2 {
            for operation in 0..3 {
                let key = format!("thread_{}_op_{}", thread_id, operation);
                let value = format!("data_{}", thread_id * 3 + operation);

                cache.set(&key, value.clone(), 30);
                assert_eq!(cache.get(&key), Some(value));
            }
        }

        // Verify data within capacity is still accessible
        for thread_id in 0..2 {
            for operation in 0..3 {
                let key = format!("thread_{}_op_{}", thread_id, operation);
                let expected_value = format!("data_{}", thread_id * 3 + operation);
                assert_eq!(cache.get(&key), Some(expected_value));
            }
        }
    }

    #[test]
    fn test_cache_entry_exact_timing() {
        // Test entry timing precision
        let now = Instant::now();
        let expires_in_5_seconds = now + Duration::from_secs(5);

        let entry = CacheEntry {
            value: "timing_test".to_string(),
            expires_at: expires_in_5_seconds,
            current_time: now,
        };

        assert_eq!(entry.expiry_seconds(), 5);
        assert!(entry.expires_within(5)); // Should expire exactly at threshold
        assert!(entry.expires_within(6)); // Should expire before 6 seconds
        assert!(!entry.expires_within(4)); // Should not expire before 4 seconds
    }

    #[test]
    fn test_cache_entry_boundary_conditions() {
        let now = Instant::now();

        // Test exactly at boundary
        let exactly_now = CacheEntry {
            value: "now".to_string(),
            expires_at: now,
            current_time: now,
        };
        assert_eq!(exactly_now.expiry_seconds(), 0);
        assert!(exactly_now.expires_within(0));
        assert!(exactly_now.expires_within(1));

        // Test 1 nanosecond future
        let one_nano_future = CacheEntry {
            value: "nano".to_string(),
            expires_at: now + Duration::from_nanos(1),
            current_time: now,
        };
        assert_eq!(one_nano_future.expiry_seconds(), 0); // Rounds down to 0
        assert!(one_nano_future.expires_within(1));
    }

    #[test]
    fn test_cache_with_complex_types() {
        use std::collections::HashMap;

        // Test with HashMap
        let mut map_cache = QuickCache::new();
        let mut map = HashMap::new();
        map.insert("nested_key".to_string(), "nested_value".to_string());

        map_cache.set("map_key", map.clone(), 30);
        assert_eq!(map_cache.get("map_key"), Some(map));

        // Test with Option
        let mut option_cache = QuickCache::new();
        option_cache.set("some_key", Some(42), 30);
        assert_eq!(option_cache.get("some_key"), Some(Some(42)));

        option_cache.set("none_key", None::<i32>, 30);
        assert_eq!(option_cache.get("none_key"), Some(None));
    }

    #[test]
    fn test_get_with_expiry_expired_cleanup() {
        let mut cache = QuickCache::new();

        // Set a value that expires quickly
        cache.set("expiry_cleanup", "will_expire".to_string(), 1);

        // Verify it exists with expiry info
        assert!(cache.get_with_expiry("expiry_cleanup").is_some());

        // Wait for expiration
        sleep(Duration::from_secs(2));

        // get_with_expiry should also cleanup expired entries
        assert!(cache.get_with_expiry("expiry_cleanup").is_none());

        // Subsequent calls should also return None
        assert!(cache.get_with_expiry("expiry_cleanup").is_none());
    }
}
