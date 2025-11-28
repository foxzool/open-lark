//! 测试辅助工具和通用函数
//!
//! 提供测试过程中常用的辅助功能，包括：
//! - 断言辅助函数
//! - 异步测试工具
//! - 性能测试工具
//! - 测试清理工具

use std::time::{Duration, Instant};
use serde_json::Value;
use tokio::time::timeout;
use serde_json::json;

/// 性能测试工具
pub struct PerformanceTest {
    start_time: Instant,
}

impl PerformanceTest {
    /// 开始性能测试
    pub fn start() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// 断言执行时间在预期范围内
    pub fn assert_duration_less_than(self, max_duration: Duration) -> Result<Self, String> {
        let elapsed = self.start_time.elapsed();
        if elapsed > max_duration {
            Err(format!(
                "Performance test failed: took {:?}, expected less than {:?}",
                elapsed, max_duration
            ))
        } else {
            Ok(self)
        }
    }

    /// 获取已消耗时间
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// 获取性能报告
    pub fn report(&self) -> String {
        format!("Execution time: {:?}", self.start_time.elapsed())
    }
}

/// 内存使用监控（简单实现）
pub struct MemoryMonitor;

impl MemoryMonitor {
    /// 记录当前内存使用情况
    pub fn snapshot() -> MemorySnapshot {
        MemorySnapshot {
            timestamp: std::time::SystemTime::now(),
            // 在实际实现中，这里应该获取真实的内存使用情况
            // 目前使用占位符值
            memory_usage: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub timestamp: std::time::SystemTime,
    pub memory_usage: usize,
}

/// 测试数据清理工具
pub struct TestCleaner;

impl TestCleaner {
    /// 清理测试创建的资源
    pub async fn cleanup_test_resources(
        app_token: &str,
        table_ids: Vec<String>,
        record_ids: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 在实际实现中，这里应该调用 API 清理创建的资源
        // 目前只是占位符实现
        println!("Cleaning up test resources:");
        println!("  App token: {}", app_token);
        println!("  Tables to delete: {:?}", table_ids);
        println!("  Records to delete: {:?}", record_ids);

        Ok(())
    }

    /// 清理过期的测试数据
    pub async fn cleanup_expired_test_data(
        older_than_days: u64,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        // 在实际实现中，这里应该清理超过指定天数的测试数据
        println!("Cleaning up test data older than {} days", older_than_days);
        Ok(0)
    }
}

/// 批量测试工具
pub struct BatchTest<T> {
    items: Vec<T>,
}

impl<T> BatchTest<T> {
    /// 创建批量测试实例
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    /// 并行执行测试
    pub async fn execute_parallel<F, Fut, R>(&self, test_fn: F) -> Vec<Result<R, Box<dyn std::error::Error>>>
    where
        F: Fn(T) -> Fut + Copy + Send + Sync,
        Fut: std::future::Future<Output = Result<R, Box<dyn std::error::Error>>> + Send,
        T: Copy + Send + Sync,
        R: Send,
    {
        let futures: Vec<_> = self.items.iter().map(|&item| test_fn(item)).collect();
        futures_util::future::join_all(futures).await
    }

    /// 串行执行测试
    pub async fn execute_sequential<F, Fut, R>(&self, test_fn: F) -> Vec<Result<R, Box<dyn std::error::Error>>>
    where
        F: Fn(&T) -> Fut,
        Fut: std::future::Future<Output = Result<R, Box<dyn std::error::Error>>>,
    {
        let mut results = Vec::new();
        for item in &self.items {
            results.push(test_fn(item).await);
        }
        results
    }
}

/// HTTP 响应验证工具
pub struct ResponseValidator;

impl ResponseValidator {
    /// 验证成功响应结构
    pub fn validate_success_response(response: &Value) -> Result<(), String> {
        if response.get("error").is_some() {
            return Err(format!("Response contains error: {:?}", response));
        }

        // 检查是否有数据字段
        if response.as_object().unwrap().is_empty() {
            return Err("Response is empty".to_string());
        }

        Ok(())
    }

    /// 验证错误响应结构
    pub fn validate_error_response(response: &Value, expected_code: i64) -> Result<(), String> {
        let error = response.get("error")
            .ok_or("Response does not contain error field")?;

        let code = error.get("code")
            .and_then(|c| c.as_i64())
            .ok_or("Error code is not a valid integer")?;

        if code != expected_code {
            return Err(format!(
                "Expected error code {}, but got {}",
                expected_code, code
            ));
        }

        Ok(())
    }

    /// 验证分页响应结构
    pub fn validate_paginated_response(response: &Value) -> Result<(), String> {
        let items = response.get("items")
            .ok_or("Missing items field")?
            .as_array()
            .ok_or("Items is not an array")?;
        let total = response.get("total")
            .ok_or("Missing total field")?
            .as_i64()
            .ok_or("Total is not an integer")?;
        let has_more = response.get("has_more")
            .ok_or("Missing has_more field")?
            .as_bool()
            .ok_or("Has_more is not a boolean")?;

        if items.len() as i64 > total {
            return Err(format!(
                "Items count ({}) exceeds total ({})",
                items.len(),
                total
            ));
        }

        if has_more && items.len() as i64 >= total {
            return Err("has_more is true but all items are returned".to_string());
        }

        Ok(())
    }
}

/// 测试辅助工具集合
pub struct TestHelper;

impl TestHelper {
    /// 验证 JSON 响应是否包含指定字段
    pub fn assert_has_field(response: &Value, field_path: &str) -> Result<(), String> {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = response;

        for part in parts {
            match current.get(part) {
                Some(value) => current = value,
                None => return Err(format!("Field '{}' not found in response", field_path)),
            }
        }

        Ok(())
    }

    /// 验证 JSON 响应字段值
    pub fn assert_field_eq(response: &Value, field_path: &str, expected: &Value) -> Result<(), String> {
        Self::assert_has_field(response, field_path)?;

        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = response;

        for part in parts {
            current = current.get(part).unwrap();
        }

        if current == expected {
            Ok(())
        } else {
            Err(format!(
                "Field '{}' expected {:?}, but got {:?}",
                field_path, expected, current
            ))
        }
    }

    /// 验证响应状态和时间戳
    pub fn assert_response_metadata(response: &Value) -> Result<(), String> {
        // 检查必要的时间戳字段
        let timestamp_fields = ["created_at", "updated_at"];
        for field in timestamp_fields.iter() {
            Self::assert_has_field(response, field)?;

            let timestamp = response.pointer(&format!("/{}", field)).unwrap();
            if let Some(ts_str) = timestamp.as_str() {
                if ts_str.is_empty() {
                    return Err(format!("Timestamp '{}' is empty", field));
                }
            } else {
                return Err(format!("Timestamp '{}' is not a string", field));
            }
        }

        Ok(())
    }

    /// 异步测试执行器，带超时控制
    pub async fn execute_with_timeout<F, T>(
        future: F,
        duration: Duration,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        F: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        match timeout(duration, future).await {
            Ok(result) => result,
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                format!("Test timed out after {:?}", duration),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_has_field() {
        let response = json!({
            "data": {
                "name": "test",
                "value": 123
            }
        });

        assert!(TestHelper::assert_has_field(&response, "data.name").is_ok());
        assert!(TestHelper::assert_has_field(&response, "data.non_existent").is_err());
    }

    #[test]
    fn test_assert_field_eq() {
        let response = json!({
            "data": {
                "name": "test",
                "value": 123
            }
        });

        assert!(TestHelper::assert_field_eq(&response, "data.name", &json!("test")).is_ok());
        assert!(TestHelper::assert_field_eq(&response, "data.value", &json!(123)).is_ok());
        assert!(TestHelper::assert_field_eq(&response, "data.name", &json!("wrong")).is_err());
    }

    #[test]
    fn test_performance_test() {
        let perf_test = PerformanceTest::start();
        std::thread::sleep(Duration::from_millis(10));

        let result = perf_test.assert_duration_less_than(Duration::from_millis(20));
        assert!(result.is_ok());

        let report = result.unwrap().report();
        assert!(report.contains("Execution time:"));
    }

    #[test]
    fn test_validate_success_response() {
        let success_response = json!({
            "data": {
                "id": "123",
                "name": "test"
            }
        });

        assert!(ResponseValidator::validate_success_response(&success_response).is_ok());

        let error_response = json!({
            "error": {
                "code": 500,
                "message": "Internal Server Error"
            }
        });

        assert!(ResponseValidator::validate_success_response(&error_response).is_err());
    }

    #[test]
    fn test_validate_error_response() {
        let error_response = json!({
            "error": {
                "code": 404,
                "message": "Not Found"
            }
        });

        assert!(ResponseValidator::validate_error_response(&error_response, 404).is_ok());
        assert!(ResponseValidator::validate_error_response(&error_response, 500).is_err());
    }

    #[test]
    fn test_validate_paginated_response() {
        let paginated_response = json!({
            "items": [
                {"id": "1"},
                {"id": "2"}
            ],
            "has_more": true,
            "total": 10
        });

        assert!(ResponseValidator::validate_paginated_response(&paginated_response).is_ok());

        let invalid_response = json!({
            "items": [{"id": "1"}],
            "has_more": true,
            "total": 1  // 错误：has_more为true但已返回所有项目
        });

        assert!(ResponseValidator::validate_paginated_response(&invalid_response).is_err());
    }

    #[tokio::test]
    async fn test_execute_with_timeout() {
        let quick_future = async { Ok::<(), Box<dyn std::error::Error>>(()) };
        let result = TestHelper::execute_with_timeout(quick_future, Duration::from_millis(100)).await;
        assert!(result.is_ok());

        let slow_future = async {
            tokio::time::sleep(Duration::from_millis(200)).await;
            Ok::<(), Box<dyn std::error::Error>>(())
        };
        let result = TestHelper::execute_with_timeout(slow_future, Duration::from_millis(100)).await;
        assert!(result.is_err());
    }
}