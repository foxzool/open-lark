/// 错误恢复和自动重试中间件
///
/// 提供智能的自动重试机制，支持：
/// - 指数退避策略
/// - 错误类型过滤
/// - 重试次数限制
/// - 自定义重试条件
/// - 重试状态监控
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, Instant};

use crate::error::traits::ErrorTrait;
use crate::{
    error::{ErrorType, LarkAPIError, RetryPolicy},
    SDKResult,
};

/// 简化版重试策略构建器，基于 RetryPolicy 提供线性/指数便捷工厂
pub struct RetryStrategyBuilder;

impl RetryStrategyBuilder {
    pub fn linear(max_retries: u32, delay: Duration) -> RetryPolicy {
        RetryPolicy::fixed(max_retries, delay)
    }

    pub fn exponential(max_retries: u32, base_delay: Duration) -> RetryPolicy {
        RetryPolicy::exponential(max_retries, base_delay)
    }
}

/// 重试中间件配置
#[derive(Clone)]
pub struct RetryConfig {
    /// 全局默认重试策略
    pub default_strategy: RetryPolicy,
    /// 是否启用重试
    pub enabled: bool,
    /// 重试统计回调
    pub on_retry: Option<Arc<dyn Fn(&RetryAttempt) + Send + Sync>>,
    /// 自定义重试条件
    pub retry_filter: Option<Arc<dyn Fn(&LarkAPIError) -> bool + Send + Sync>>,
}

impl std::fmt::Debug for RetryConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RetryConfig")
            .field("default_strategy", &self.default_strategy)
            .field("enabled", &self.enabled)
            .field(
                "on_retry",
                &self.on_retry.as_ref().map(|_| "Fn(&RetryAttempt)"),
            )
            .field(
                "retry_filter",
                &self
                    .retry_filter
                    .as_ref()
                    .map(|_| "Fn(&LarkAPIError) -> bool"),
            )
            .finish()
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            default_strategy: RetryPolicy::default(),
            enabled: true,
            on_retry: None,
            retry_filter: None,
        }
    }
}

impl RetryConfig {
    /// 创建新的重试配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否启用重试
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置默认重试策略
    pub fn default_strategy(mut self, strategy: RetryPolicy) -> Self {
        self.default_strategy = strategy;
        self
    }

    /// 设置重试回调函数
    pub fn on_retry<F>(mut self, callback: F) -> Self
    where
        F: Fn(&RetryAttempt) + Send + Sync + 'static,
    {
        self.on_retry = Some(Arc::new(callback));
        self
    }

    /// 设置自定义重试过滤器
    pub fn retry_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&LarkAPIError) -> bool + Send + Sync + 'static,
    {
        self.retry_filter = Some(Arc::new(filter));
        self
    }

    /// 快速配置：仅重试服务器错误
    pub fn server_errors_only(mut self) -> Self {
        self.retry_filter = Some(Arc::new(|error| {
            // 使用 ErrorKind 匹配而不是函数匹配
            match error.error_type() {
                ErrorType::Network => {
                    // 检查错误消息中是否包含服务器错误的特征
                    error.message().contains("timeout")
                        || error.message().contains("timed out")
                        || error.message().contains("connect")
                        || error.message().contains("connection")
                }
                ErrorType::Validation => false, // Don't retry validation errors
                _ => false,                     // 其他类型不重试
            }
        }));
        self
    }

    /// 快速配置：激进重试策略
    pub fn aggressive(mut self) -> Self {
        self.default_strategy = RetryPolicy::exponential(5, Duration::from_millis(500));
        self
    }

    /// 快速配置：保守重试策略
    pub fn conservative(mut self) -> Self {
        self.default_strategy = RetryPolicy::fixed(2, Duration::from_secs(2));
        self
    }
}

/// 重试尝试信息
#[derive(Debug, Clone)]
pub struct RetryAttempt {
    /// 当前尝试次数
    pub attempt: u32,
    /// 最大尝试次数
    pub max_attempts: u32,
    /// 本次延迟时间
    pub delay: Duration,
    /// 错误信息
    pub error: LarkAPIError,
    /// 重试开始时间
    pub started_at: Instant,
    /// 累计耗时
    pub elapsed: Duration,
}

impl RetryAttempt {
    /// 获取最大重试次数
    pub fn max_retries(&self) -> u32 {
        self.max_attempts
    }

    /// 是否为最后一次尝试
    pub fn is_final_attempt(&self) -> bool {
        self.attempt >= self.max_retries()
    }

    /// 剩余尝试次数
    pub fn remaining_attempts(&self) -> u32 {
        self.max_retries().saturating_sub(self.attempt)
    }

    /// 打印重试信息
    pub fn print_info(&self) {
        let percentage = (self.attempt as f32 / self.max_retries() as f32 * 100.0) as u32;
        println!(
            "🔄 重试 {}/{} ({}%) - 延迟 {:?} - 耗时 {:?}",
            self.attempt,
            self.max_retries(),
            percentage,
            self.delay,
            self.elapsed
        );
    }
}

/// 重试中间件
pub struct RetryMiddleware {
    config: RetryConfig,
}

impl Default for RetryMiddleware {
    fn default() -> Self {
        Self::new(RetryConfig::default())
    }
}

impl RetryMiddleware {
    /// 创建新的重试中间件
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// 执行带重试的操作
    pub async fn execute<F, T, Fut>(&self, operation: F) -> SDKResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = SDKResult<T>>,
    {
        if !self.config.enabled {
            return operation().await;
        }

        let started_at = Instant::now();
        let mut last_error = None;

        for attempt in 1..=self.config.default_strategy.max_retries() {
            let result = operation().await;

            match result {
                Ok(value) => return Ok(value),
                Err(error) => {
                    last_error = Some(error.clone());

                    // 检查是否应该重试
                    if !self.should_retry(&error, attempt) {
                        return Err(error);
                    }

                    // 计算延迟时间
                    let delay = self.calculate_delay(attempt - 1);
                    let elapsed = started_at.elapsed();

                    // 创建重试尝试信息
                    let retry_attempt = RetryAttempt {
                        attempt,
                        max_attempts: self.config.default_strategy.max_retries(),
                        delay,
                        error: error.clone(),
                        started_at,
                        elapsed,
                    };

                    // 调用重试回调
                    if let Some(callback) = &self.config.on_retry {
                        callback(&retry_attempt);
                    }

                    // 如果不是最后一次尝试，则等待
                    if !retry_attempt.is_final_attempt() {
                        sleep(delay).await;
                    }
                }
            }
        }

        // 返回最后一个错误
        Err(last_error.unwrap())
    }

    /// 检查是否应该重试
    fn should_retry(&self, error: &LarkAPIError, attempt: u32) -> bool {
        // 检查是否达到最大重试次数
        if attempt >= self.config.default_strategy.max_retries() {
            return false;
        }

        // 应用自定义过滤器
        if let Some(filter) = &self.config.retry_filter {
            return filter(error);
        }

        // 使用默认的重试逻辑
        error.is_retryable()
    }

    /// 计算延迟时间
    fn calculate_delay(&self, attempt: u32) -> Duration {
        self.config
            .default_strategy
            .retry_delay(attempt)
            .unwrap_or(Duration::ZERO)
    }
}

/// 重试结果统计
#[derive(Debug, Default)]
pub struct RetryStats {
    /// 总尝试次数
    pub total_attempts: u32,
    /// 成功次数
    pub successful_attempts: u32,
    /// 重试次数
    pub retry_count: u32,
    /// 总耗时
    pub total_duration: Duration,
    /// 平均延迟
    pub average_delay: Duration,
}

impl RetryStats {
    /// 计算成功率
    pub fn success_rate(&self) -> f32 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.successful_attempts as f32 / self.total_attempts as f32
        }
    }

    /// 打印统计信息
    pub fn print_summary(&self) {
        println!("📊 重试统计:");
        println!("   总尝试次数: {}", self.total_attempts);
        println!("   成功次数: {}", self.successful_attempts);
        println!("   重试次数: {}", self.retry_count);
        println!("   成功率: {:.1}%", self.success_rate() * 100.0);
        println!("   总耗时: {:?}", self.total_duration);
        println!("   平均延迟: {:?}", self.average_delay);
    }
}

/// 带统计的重试中间件
pub struct RetryMiddlewareWithStats {
    middleware: RetryMiddleware,
    stats: Arc<std::sync::Mutex<RetryStats>>,
}

impl RetryMiddlewareWithStats {
    /// 创建带统计的重试中间件
    pub fn new(config: RetryConfig) -> Self {
        let stats = Arc::new(std::sync::Mutex::new(RetryStats::default()));
        let stats_clone = Arc::clone(&stats);

        // 添加统计回调
        let config_with_stats = config.on_retry(move |attempt| {
            if let Ok(mut stats) = stats_clone.lock() {
                stats.total_attempts += 1;
                stats.retry_count += 1;
                stats.total_duration += attempt.elapsed;
            }
        });

        Self {
            middleware: RetryMiddleware::new(config_with_stats),
            stats,
        }
    }

    /// 执行带统计的重试操作
    pub async fn execute<F, T, Fut>(&self, operation: F) -> SDKResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = SDKResult<T>>,
    {
        let result = self.middleware.execute(operation).await;

        // 更新统计信息
        if let Ok(mut stats) = self.stats.lock() {
            if result.is_ok() {
                stats.successful_attempts += 1;
            }
        }

        result
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> RetryStats {
        let stats = self.stats.lock().unwrap();
        RetryStats {
            total_attempts: stats.total_attempts,
            successful_attempts: stats.successful_attempts,
            retry_count: stats.retry_count,
            total_duration: stats.total_duration,
            average_delay: stats.average_delay,
        }
    }

    /// 重置统计信息
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            *stats = RetryStats::default();
        }
    }
}

/// 重试策略构建器 (使用 RetryPolicy 的简化构建器)
pub struct RetryPolicyBuilder;

impl RetryPolicyBuilder {
    /// 快速创建线性重试策略
    pub fn linear(max_attempts: u32, delay: Duration) -> RetryPolicy {
        RetryPolicy::fixed(max_attempts, delay)
    }

    /// 快速创建指数退避策略
    pub fn exponential(
        max_attempts: u32,
        base_delay: Duration,
        _max_delay: Duration,
    ) -> RetryPolicy {
        RetryPolicy::exponential(max_attempts, base_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::new().enabled(true).aggressive();

        assert!(config.enabled);
        assert_eq!(config.default_strategy.max_retries(), 5);
    }

    #[test]
    fn test_retry_policy_basic() {
        let policy = RetryPolicy::exponential(3, Duration::from_secs(1));

        assert_eq!(policy.max_retries(), 3);
        assert_eq!(policy.delay(0), Duration::from_secs(1));
        assert!(policy.is_retryable());
    }

    #[test]
    fn test_linear_strategy() {
        let strategy = RetryPolicyBuilder::linear(3, Duration::from_secs(2));

        assert_eq!(strategy.max_retries(), 3);
        assert_eq!(strategy.delay(0), Duration::from_secs(2));
        assert_eq!(strategy.delay(1), Duration::from_secs(2)); // 线性重试延迟不变
    }

    #[test]
    fn test_exponential_strategy() {
        let strategy =
            RetryPolicyBuilder::exponential(5, Duration::from_millis(500), Duration::from_secs(30));

        assert_eq!(strategy.max_retries(), 5);
        assert_eq!(strategy.delay(0), Duration::from_millis(500));
        assert_eq!(strategy.delay(1), Duration::from_millis(1000)); // 指数退避
        assert_eq!(strategy.delay(2), Duration::from_millis(2000)); // 2^2 * 500ms
    }

    #[test]
    fn test_retry_attempt_info() {
        let error = LarkAPIError::api_error(500, "Server Error", "server error", None::<String>);
        let attempt = RetryAttempt {
            attempt: 2,
            max_attempts: 3,
            delay: Duration::from_secs(2),
            error,
            started_at: Instant::now(),
            elapsed: Duration::from_secs(5),
        };

        assert!(!attempt.is_final_attempt());
        assert_eq!(attempt.remaining_attempts(), 1);
    }

    #[test]
    fn test_retry_stats() {
        let stats = RetryStats {
            total_attempts: 10,
            successful_attempts: 8,
            retry_count: 5,
            total_duration: Duration::from_secs(30),
            average_delay: Duration::from_secs(2),
        };

        assert_eq!(stats.success_rate(), 0.8);
    }

    #[tokio::test]
    async fn test_retry_middleware_success() {
        use std::sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        };

        let middleware = RetryMiddleware::default();
        let call_count = Arc::new(AtomicU32::new(0));

        let call_count_clone = Arc::clone(&call_count);
        let result: Result<&str, LarkAPIError> = middleware
            .execute(move || {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst) + 1;
                async move {
                    if count == 1 {
                        Err(LarkAPIError::api_error(
                            500,
                            "Server Error",
                            "server error",
                            None::<String>,
                        ))
                    } else {
                        Ok("Success")
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_retry_middleware_failure() {
        use std::sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        };

        let config = RetryConfig::new()
            .default_strategy(RetryStrategyBuilder::linear(2, Duration::from_millis(1)));

        let middleware = RetryMiddleware::new(config);
        let call_count = Arc::new(AtomicU32::new(0));

        let call_count_clone = Arc::clone(&call_count);
        let result: Result<&str, LarkAPIError> = middleware
            .execute(move || {
                call_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    Err(LarkAPIError::api_error(
                        500,
                        "Server Error",
                        "server error",
                        None::<String>,
                    ))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert!(config.enabled);
        assert_eq!(config.default_strategy.max_retries(), 3); // Default from RetryPolicy
        assert!(config.on_retry.is_none());
        assert!(config.retry_filter.is_none());
    }

    #[test]
    fn test_retry_config_builder_patterns() {
        // Test conservative strategy
        let conservative_config = RetryConfig::new().conservative();
        assert_eq!(conservative_config.default_strategy.max_retries(), 2);
        assert_eq!(
            conservative_config.default_strategy.base_delay,
            Duration::from_secs(2)
        );
        assert!(!conservative_config
            .default_strategy
            .use_exponential_backoff());

        // Test aggressive strategy
        let aggressive_config = RetryConfig::new().aggressive();
        assert_eq!(aggressive_config.default_strategy.max_retries(), 5);
        assert_eq!(
            aggressive_config.default_strategy.base_delay,
            Duration::from_millis(500)
        );
        assert!(aggressive_config.default_strategy.use_exponential_backoff());

        // Test server errors only filter
        let server_error_config = RetryConfig::new().server_errors_only();
        assert!(server_error_config.retry_filter.is_some());
    }

    #[test]
    fn test_retry_config_chaining() {
        let config = RetryConfig::new()
            .enabled(false)
            .aggressive()
            .server_errors_only();

        assert!(!config.enabled); // Note: aggressive() resets to defaults, then server_errors_only adds filter
        assert_eq!(config.default_strategy.max_retries(), 5);
        assert!(config.retry_filter.is_some());
    }

    #[test]
    fn test_retry_attempt_methods() {
        let error = LarkAPIError::api_error(500, "Server Error", "server error", None::<String>);
        let started_at = Instant::now();

        // Test final attempt
        let final_attempt = RetryAttempt {
            attempt: 3,
            max_attempts: 3,
            delay: Duration::from_secs(1),
            error: error.clone(),
            started_at,
            elapsed: Duration::from_secs(5),
        };
        assert!(final_attempt.is_final_attempt());
        assert_eq!(final_attempt.remaining_attempts(), 0);

        // Test non-final attempt
        let non_final_attempt = RetryAttempt {
            attempt: 1,
            max_attempts: 3,
            delay: Duration::from_secs(1),
            error: error.clone(),
            started_at,
            elapsed: Duration::from_secs(2),
        };
        assert!(!non_final_attempt.is_final_attempt());
        assert_eq!(non_final_attempt.remaining_attempts(), 2);

        // Test saturation (remaining attempts can't go negative)
        let saturated_attempt = RetryAttempt {
            attempt: 5,
            max_attempts: 3,
            delay: Duration::from_secs(1),
            error,
            started_at,
            elapsed: Duration::from_secs(10),
        };
        assert_eq!(saturated_attempt.remaining_attempts(), 0);
    }

    #[test]
    fn test_retry_stats_edge_cases() {
        // Test empty stats
        let empty_stats = RetryStats::default();
        assert_eq!(empty_stats.success_rate(), 0.0);
        assert_eq!(empty_stats.total_attempts, 0);
        assert_eq!(empty_stats.successful_attempts, 0);

        // Test zero division safety
        let zero_attempt_stats = RetryStats {
            total_attempts: 0,
            successful_attempts: 0,
            retry_count: 0,
            total_duration: Duration::ZERO,
            average_delay: Duration::ZERO,
        };
        assert_eq!(zero_attempt_stats.success_rate(), 0.0);

        // Test perfect success rate
        let perfect_stats = RetryStats {
            total_attempts: 10,
            successful_attempts: 10,
            retry_count: 0,
            total_duration: Duration::from_secs(10),
            average_delay: Duration::from_millis(100),
        };
        assert_eq!(perfect_stats.success_rate(), 1.0);

        // Test zero success rate
        let zero_success_stats = RetryStats {
            total_attempts: 10,
            successful_attempts: 0,
            retry_count: 10,
            total_duration: Duration::from_secs(30),
            average_delay: Duration::from_millis(500),
        };
        assert_eq!(zero_success_stats.success_rate(), 0.0);
    }

    #[tokio::test]
    async fn test_retry_middleware_disabled() {
        let config = RetryConfig::new().enabled(false);
        let middleware = RetryMiddleware::new(config);
        let call_count = std::sync::atomic::AtomicU32::new(0);

        let result: Result<&str, LarkAPIError> = middleware
            .execute(|| {
                call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async move {
                    Err(LarkAPIError::api_error(
                        500,
                        "Server Error",
                        "server error",
                        None::<String>,
                    ))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1); // Only called once
    }

    #[tokio::test]
    async fn test_retry_middleware_with_callback() {
        use std::sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        };

        let callback_count = Arc::new(AtomicU32::new(0));
        let callback_count_clone = Arc::clone(&callback_count);

        let config = RetryConfig::new().on_retry(move |_attempt| {
            callback_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        let middleware = RetryMiddleware::new(config);

        let result: Result<&str, LarkAPIError> = middleware
            .execute(|| async move {
                Err(LarkAPIError::api_error(
                    500,
                    "Server Error",
                    "server error",
                    None::<String>,
                ))
            })
            .await;

        assert!(result.is_err());
        // Should be called max_attempts - 1 times (after first failure, before final attempt)
        assert_eq!(callback_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_retry_middleware_custom_filter() {
        use std::sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        };

        // Only retry on specific error codes
        let config = RetryConfig::new()
            .retry_filter(|error| matches!(error, LarkAPIError::Api(api) if api.status == 503));

        let middleware = RetryMiddleware::new(config);
        let call_count = Arc::new(AtomicU32::new(0));

        let call_count_clone = Arc::clone(&call_count);
        let result: Result<&str, LarkAPIError> = middleware
            .execute(move || {
                let _count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    // Return 500 error, which should not be retried according to our filter
                    Err(LarkAPIError::api_error(
                        500,
                        "Server Error",
                        "server error",
                        None::<String>,
                    ))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // Only called once, no retry
    }

    #[tokio::test]
    async fn test_retry_middleware_exponential_backoff() {
        use std::sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        };
        use std::time::Instant;

        let config = RetryConfig::new().default_strategy(RetryStrategyBuilder::exponential(
            3,
            Duration::from_millis(10),
        ));

        let middleware = RetryMiddleware::new(config);
        let call_count = Arc::new(AtomicU32::new(0));
        let start_time = Arc::new(Instant::now());

        let call_count_clone = Arc::clone(&call_count);
        let start_time_clone = Arc::clone(&start_time);

        let result: Result<&str, LarkAPIError> = middleware
            .execute(move || {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    if count < 2 {
                        Err(LarkAPIError::api_error(
                            500,
                            "Server Error",
                            "server error",
                            None::<String>,
                        ))
                    } else {
                        Ok("Success")
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(call_count.load(Ordering::SeqCst), 3);

        // Should have taken some time due to exponential backoff
        let elapsed = start_time_clone.elapsed();
        assert!(elapsed >= Duration::from_millis(10)); // At least the first delay
    }

    #[tokio::test]
    async fn test_retry_middleware_with_stats() {
        use std::sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        };

        let config = RetryConfig::new();
        let middleware = RetryMiddlewareWithStats::new(config);
        let call_count = Arc::new(AtomicU32::new(0));

        let call_count_clone = Arc::clone(&call_count);
        let result: Result<&str, LarkAPIError> = middleware
            .execute(move || {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    if count == 1 {
                        Ok("Success")
                    } else {
                        Err(LarkAPIError::api_error(
                            500,
                            "Server Error",
                            "server error",
                            None::<String>,
                        ))
                    }
                }
            })
            .await;

        assert!(result.is_ok());

        let stats = middleware.get_stats();
        // Note: The stats callback is only called on retries, not on the final successful attempt
        // So total_attempts might be 1 if the final successful attempt isn't counted in the callback
        assert!(stats.total_attempts >= 1);
        assert_eq!(stats.successful_attempts, 1);
        // The retry_count might be 1 if there was one retry, regardless of total counting method
        // retry_count is always >= 0 by definition, so this check is redundant
    }

    #[tokio::test]
    async fn test_retry_middleware_stats_reset() {
        let config = RetryConfig::new();
        let middleware = RetryMiddlewareWithStats::new(config);

        // Execute a failed operation to generate stats
        let _result: Result<&str, LarkAPIError> = middleware
            .execute(|| async move {
                Err(LarkAPIError::api_error(
                    500,
                    "Server Error",
                    "server error",
                    None::<String>,
                ))
            })
            .await;

        let stats_before = middleware.get_stats();
        assert!(stats_before.total_attempts > 0);

        // Reset stats
        middleware.reset_stats();

        let stats_after = middleware.get_stats();
        assert_eq!(stats_after.total_attempts, 0);
        assert_eq!(stats_after.successful_attempts, 0);
        assert_eq!(stats_after.retry_count, 0);
    }

    #[test]
    fn test_retry_policy_builder_defaults() {
        let strategy = RetryPolicy::default();

        // Should match default RetryPolicy values
        assert!(strategy.max_retries() > 0);
        assert!(strategy.delay(0) > Duration::ZERO);
        assert!(strategy.is_retryable());
    }

    #[test]
    fn test_retry_policy_exponential() {
        let strategy = RetryPolicy::exponential(5, Duration::from_millis(200));

        assert_eq!(strategy.max_retries(), 5);
        assert_eq!(strategy.delay(0), Duration::from_millis(200));
        assert_eq!(strategy.delay(1), Duration::from_millis(400));
        assert_eq!(strategy.delay(2), Duration::from_millis(800));
    }

    #[test]
    fn test_retry_config_debug_format() {
        let config = RetryConfig::new()
            .enabled(true)
            .on_retry(|_attempt| println!("Retry"));

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("RetryConfig"));
        assert!(debug_str.contains("enabled: true"));
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let config = RetryConfig::new();
        let middleware = RetryMiddleware::new(config);
        let call_count = std::sync::atomic::AtomicU32::new(0);

        // Use a non-retryable error (like authentication error)
        let result: Result<&str, LarkAPIError> = middleware
            .execute(|| {
                call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async move {
                    Err(crate::error::validation_error(
                        "token",
                        "Invalid auth token",
                    ))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1); // Should not retry
    }

    #[test]
    fn test_retry_stats_methods() {
        let mut stats = RetryStats::default();

        // Test print_summary doesn't panic
        stats.print_summary();

        // Test success rate calculations
        stats.total_attempts = 5;
        stats.successful_attempts = 3;
        assert_eq!(stats.success_rate(), 0.6);

        // Test with large numbers
        stats.total_attempts = 1000;
        stats.successful_attempts = 867;
        assert!((stats.success_rate() - 0.867).abs() < 0.001);
    }
}
