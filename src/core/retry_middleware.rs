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

use crate::core::{error::LarkAPIError, error_helper::RetryStrategy, SDKResult};

/// 重试中间件配置
#[derive(Clone)]
pub struct RetryConfig {
    /// 全局默认重试策略
    pub default_strategy: RetryStrategy,
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
            default_strategy: RetryStrategy::default(),
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
    pub fn default_strategy(mut self, strategy: RetryStrategy) -> Self {
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
        self.retry_filter = Some(Arc::new(|error| match error {
            LarkAPIError::ApiError { code, .. } => {
                matches!(*code, 500..=599)
            }
            LarkAPIError::RequestError(req_err) => {
                req_err.contains("timeout")
                    || req_err.contains("timed out")
                    || req_err.contains("connect")
                    || req_err.contains("connection")
            }
            _ => false,
        }));
        self
    }

    /// 快速配置：激进重试策略
    pub fn aggressive(mut self) -> Self {
        self.default_strategy = RetryStrategy {
            max_attempts: 5,
            base_delay: Duration::from_millis(500),
            use_exponential_backoff: true,
            max_delay: Duration::from_secs(30),
        };
        self
    }

    /// 快速配置：保守重试策略
    pub fn conservative(mut self) -> Self {
        self.default_strategy = RetryStrategy {
            max_attempts: 2,
            base_delay: Duration::from_secs(2),
            use_exponential_backoff: false,
            max_delay: Duration::from_secs(10),
        };
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
    /// 是否为最后一次尝试
    pub fn is_final_attempt(&self) -> bool {
        self.attempt >= self.max_attempts
    }

    /// 剩余尝试次数
    pub fn remaining_attempts(&self) -> u32 {
        self.max_attempts.saturating_sub(self.attempt)
    }

    /// 打印重试信息
    pub fn print_info(&self) {
        let percentage = (self.attempt as f32 / self.max_attempts as f32 * 100.0) as u32;
        println!(
            "🔄 重试 {}/{} ({}%) - 延迟 {:?} - 耗时 {:?}",
            self.attempt, self.max_attempts, percentage, self.delay, self.elapsed
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

        for attempt in 1..=self.config.default_strategy.max_attempts {
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
                        max_attempts: self.config.default_strategy.max_attempts,
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
        if attempt >= self.config.default_strategy.max_attempts {
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
        self.config.default_strategy.calculate_delay(attempt)
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

/// 重试策略构建器
pub struct RetryStrategyBuilder {
    strategy: RetryStrategy,
}

impl RetryStrategyBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            strategy: RetryStrategy::default(),
        }
    }

    /// 设置最大重试次数
    pub fn max_attempts(mut self, max_attempts: u32) -> Self {
        self.strategy.max_attempts = max_attempts;
        self
    }

    /// 设置基础延迟
    pub fn base_delay(mut self, delay: Duration) -> Self {
        self.strategy.base_delay = delay;
        self
    }

    /// 设置最大延迟
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.strategy.max_delay = delay;
        self
    }

    /// 启用指数退避
    pub fn exponential_backoff(mut self, enabled: bool) -> Self {
        self.strategy.use_exponential_backoff = enabled;
        self
    }

    /// 构建策略
    pub fn build(self) -> RetryStrategy {
        self.strategy
    }

    /// 快速创建线性重试策略
    pub fn linear(max_attempts: u32, delay: Duration) -> RetryStrategy {
        Self::new()
            .max_attempts(max_attempts)
            .base_delay(delay)
            .exponential_backoff(false)
            .build()
    }

    /// 快速创建指数退避策略
    pub fn exponential(
        max_attempts: u32,
        base_delay: Duration,
        max_delay: Duration,
    ) -> RetryStrategy {
        Self::new()
            .max_attempts(max_attempts)
            .base_delay(base_delay)
            .max_delay(max_delay)
            .exponential_backoff(true)
            .build()
    }
}

impl Default for RetryStrategyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::new().enabled(true).aggressive();

        assert!(config.enabled);
        assert_eq!(config.default_strategy.max_attempts, 5);
    }

    #[test]
    fn test_retry_strategy_builder() {
        let strategy = RetryStrategyBuilder::new()
            .max_attempts(3)
            .base_delay(Duration::from_secs(1))
            .exponential_backoff(true)
            .build();

        assert_eq!(strategy.max_attempts, 3);
        assert_eq!(strategy.base_delay, Duration::from_secs(1));
        assert!(strategy.use_exponential_backoff);
    }

    #[test]
    fn test_linear_strategy() {
        let strategy = RetryStrategyBuilder::linear(3, Duration::from_secs(2));

        assert_eq!(strategy.max_attempts, 3);
        assert_eq!(strategy.base_delay, Duration::from_secs(2));
        assert!(!strategy.use_exponential_backoff);
    }

    #[test]
    fn test_exponential_strategy() {
        let strategy = RetryStrategyBuilder::exponential(
            5,
            Duration::from_millis(500),
            Duration::from_secs(30),
        );

        assert_eq!(strategy.max_attempts, 5);
        assert_eq!(strategy.base_delay, Duration::from_millis(500));
        assert_eq!(strategy.max_delay, Duration::from_secs(30));
        assert!(strategy.use_exponential_backoff);
    }

    #[test]
    fn test_retry_attempt_info() {
        let error = LarkAPIError::api_error(500, "Server Error", None);
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
                        Err(LarkAPIError::api_error(500, "Server Error", None))
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
                async move { Err(LarkAPIError::api_error(500, "Server Error", None)) }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }
}
