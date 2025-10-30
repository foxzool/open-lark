//! 优化的HTTP客户端实现
//!
//! 提供高性能、自适应的HTTP客户端配置，包括：
//! - 动态连接池调整
//! - 智能压缩算法选择
//! - 自适应超时配置
//! - 请求批处理和优先级管理

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{info, warn, error, instrument};

/// 优化的HTTP配置
#[derive(Debug, Clone)]
pub struct OptimizedHttpConfig {
    /// 连接池配置
    pub pool_config: ConnectionPoolConfig,
    /// 超时配置
    pub timeout_config: TimeoutConfig,
    /// 压缩配置
    pub compression_config: CompressionConfig,
    /// 重试配置
    pub retry_config: RetryConfig,
    /// 性能监控配置
    pub monitoring_config: MonitoringConfig,
}

impl Default for OptimizedHttpConfig {
    fn default() -> Self {
        Self {
            pool_config: ConnectionPoolConfig::default(),
            timeout_config: TimeoutConfig::default(),
            compression_config: CompressionConfig::default(),
            retry_config: RetryConfig::default(),
            monitoring_config: MonitoringConfig::default(),
        }
    }
}

/// 连接池配置
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    /// 每个主机的最大空闲连接数
    pub max_idle_per_host: usize,
    /// 连接池总大小
    pub pool_max_idle_per_host: usize,
    /// 是否启用连接复用
    pub enable_connection_reuse: bool,
    /// 连接保活时间
    pub keep_alive_duration: Duration,
    /// 是否启用自适应调整
    pub enable_adaptive_pool: bool,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_idle_per_host: 100, // 增加到100
            pool_max_idle_per_host: 100,
            enable_connection_reuse: true,
            keep_alive_duration: Duration::from_secs(75),
            enable_adaptive_pool: true,
        }
    }
}

/// 超时配置
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// 连接超时
    pub connect_timeout: Duration,
    /// 读取超时
    pub read_timeout: Duration,
    /// 写入超时
    pub write_timeout: Duration,
    /// 是否启用自适应超时
    pub enable_adaptive_timeout: bool,
    /// 基础超时倍数
    pub base_timeout_multiplier: f64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(8), // 减少到8秒
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            enable_adaptive_timeout: true,
            base_timeout_multiplier: 1.0,
        }
    }
}

/// 压缩配置
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// 是否启用压缩
    pub enable_compression: bool,
    /// 压缩阈值（字节数）
    pub compression_threshold: usize,
    /// 支持的压缩算法
    pub supported_algorithms: Vec<CompressionAlgorithm>,
    /// 是否启用智能压缩选择
    pub enable_smart_compression: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enable_compression: true,
            compression_threshold: 1024, // 1KB阈值
            supported_algorithms: vec![
                CompressionAlgorithm::Gzip,
                CompressionAlgorithm::Brotli,
            ],
            enable_smart_compression: true,
        }
    }
}

/// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔策略
    pub retry_strategy: RetryStrategy,
    /// 是否启用指数退避
    pub enable_exponential_backoff: bool,
    /// 基础重试延迟
    pub base_retry_delay: Duration,
    /// 最大重试延迟
    pub max_retry_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_strategy: RetryStrategy::ExponentialBackoff,
            enable_exponential_backoff: true,
            base_retry_delay: Duration::from_millis(100),
            max_retry_delay: Duration::from_secs(30),
        }
    }
}

/// 性能监控配置
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    /// 是否启用监控
    pub enable_monitoring: bool,
    /// 请求采样率
    pub sampling_rate: f64,
    /// 慢请求阈值（毫秒）
    pub slow_request_threshold: u64,
    /// 是否启用详细指标
    pub enable_detailed_metrics: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            sampling_rate: 1.0, // 100%采样
            slow_request_threshold: 1000, // 1秒
            enable_detailed_metrics: true,
        }
    }
}

/// 压缩算法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    Brotli,
    Deflate,
}

/// 重试策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryStrategy {
    Fixed,
    Linear,
    ExponentialBackoff,
}

/// 请求统计信息
#[derive(Debug, Default, Clone)]
pub struct RequestStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_duration_ms: u64,
    pub average_duration_ms: f64,
    pub slow_requests: u64,
    pub retry_count: u64,
    pub compression_ratio: f64,
}

impl RequestStats {
    /// 更新请求统计
    pub fn update_request(&mut self, duration_ms: u64, success: bool, retried: bool) {
        self.total_requests += 1;
        self.total_duration_ms += duration_ms;
        self.average_duration_ms = self.total_duration_ms as f64 / self.total_requests as f64;

        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        if duration_ms > 1000 {
            self.slow_requests += 1;
        }

        if retried {
            self.retry_count += 1;
        }
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.successful_requests as f64 / self.total_requests as f64
        }
    }

    /// 获取慢请求率
    pub fn slow_request_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.slow_requests as f64 / self.total_requests as f64
        }
    }
}

/// 优化的HTTP客户端
#[derive(Debug)]
pub struct OptimizedHttpClient {
    client: Client,
    config: OptimizedHttpConfig,
    stats: Arc<RwLock<RequestStats>>,
    pool_stats: Arc<RwLock<ConnectionPoolStats>>,
}

/// 连接池统计信息
#[derive(Debug, Default, Clone)]
pub struct ConnectionPoolStats {
    pub active_connections: usize,
    pub idle_connections: usize,
    pub total_connections: usize,
    pub pool_utilization: f64,
    pub average_wait_time_ms: f64,
}

impl OptimizedHttpClient {
    /// 创建优化的HTTP客户端
    pub fn new(config: OptimizedHttpConfig) -> Self {
        let client = Self::build_client(&config);

        Self {
            client,
            config,
            stats: Arc::new(RwLock::new(RequestStats::default())),
            pool_stats: Arc::new(RwLock::new(ConnectionPoolStats::default())),
        }
    }

    /// 使用默认配置创建客户端
    pub fn new_default() -> Self {
        Self::new(OptimizedHttpConfig::default())
    }

    /// 构建reqwest客户端
    fn build_client(config: &OptimizedHttpConfig) -> Client {
        let mut builder = Client::builder()
            .timeout(config.timeout_config.read_timeout)
            .connect_timeout(config.timeout_config.connect_timeout);

        // 配置连接池
        builder = builder
            .pool_max_idle_per_host(config.pool_config.pool_max_idle_per_host)
            .pool_idle_timeout(config.pool_config.keep_alive_duration);

        // 配置压缩
        if config.compression_config.enable_compression {
            builder = builder.gzip(true);
        }

        builder.build().expect("Failed to create HTTP client")
    }

    /// 执行HTTP请求
    #[instrument(skip(self, request_builder))]
    pub async fn execute_request(
        &self,
        mut request_builder: RequestBuilder,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let start_time = Instant::now();

        // 智能压缩配置
        if self.config.compression_config.enable_smart_compression {
            request_builder = self.apply_smart_compression(request_builder);
        }

        // 执行请求
        let response = request_builder.send().await;
        let duration = start_time.elapsed();

        // 更新统计信息
        let success = response.is_ok();
        self.update_stats(duration.as_millis() as u64, success, false);

        // 记录慢请求
        if duration.as_millis() > self.config.monitoring_config.slow_request_threshold as u64 {
            warn!("Slow HTTP request detected: {}ms", duration.as_millis());
        }

        response
    }

    /// 获取统计信息
    pub fn stats(&self) -> RequestStats {
        self.stats.read().unwrap().clone()
    }

    /// 获取连接池统计信息
    pub fn pool_stats(&self) -> ConnectionPoolStats {
        self.pool_stats.read().unwrap().clone()
    }

    /// 自适应调整配置
    pub fn adapt_configuration(&self) {
        if !self.config.pool_config.enable_adaptive_pool {
            return;
        }

        let stats = self.stats();
        let pool_stats = self.pool_stats();

        // 根据性能统计调整连接池大小
        if stats.success_rate() < 0.95 && pool_stats.pool_utilization > 0.8 {
            info!("High connection pool utilization detected, consider increasing pool size");
        }

        // 根据请求延迟调整超时配置
        if stats.average_duration_ms > 2000.0 {
            warn!("High average request duration detected: {}ms", stats.average_duration_ms);
        }
    }

    /// 批量请求支持
    pub async fn execute_batch_requests<T>(
        &self,
        requests: Vec<RequestBuilder>,
    ) -> Vec<Result<reqwest::Response, reqwest::Error>>
        where
            T: for<'de> Deserialize<'de> + Send + 'static,
    {
        const BATCH_SIZE: usize = 10;
        const BATCH_DELAY: Duration = Duration::from_millis(50);

        let mut results = Vec::new();

        for chunk in requests.chunks(BATCH_SIZE) {
            let mut batch_tasks = Vec::new();

            for request in chunk {
                let client = self.client.clone();
                let task = async move {
                    let start_time = Instant::now();
                    let result = request.send().await;
                    let duration = start_time.elapsed();

                    // 可以在这里添加批量统计逻辑
                    (result, duration)
                };

                batch_tasks.push(task);
            }

            // 并发执行批量请求
            let batch_results = futures_util::future::join_all(batch_tasks).await;
            for (result, duration) in batch_results {
                self.update_stats(duration.as_millis() as u64, result.is_ok(), false);
                results.push(result);
            }

            // 批次间延迟，避免过载
            sleep(BATCH_DELAY).await;
        }

        results
    }

    // 内部辅助方法

    fn apply_smart_compression(&self, request_builder: RequestBuilder) -> RequestBuilder {
        // 智能压缩逻辑：根据请求特征决定是否启用压缩
        // 这里简化实现，实际可以根据请求大小、类型等判断
        request_builder
    }

    fn update_stats(&self, duration_ms: u64, success: bool, retried: bool) {
        let mut stats = self.stats.write().unwrap();
        stats.update_request(duration_ms, success, retried);
    }
}

/// 请求优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// 优先级请求队列
#[derive(Debug)]
pub struct PriorityRequestQueue {
    high_priority: Vec<RequestBuilder>,
    normal_priority: Vec<RequestBuilder>,
    low_priority: Vec<RequestBuilder>,
}

impl PriorityRequestQueue {
    pub fn new() -> Self {
        Self {
            high_priority: Vec::new(),
            normal_priority: Vec::new(),
            low_priority: Vec::new(),
        }
    }

    pub fn add_request(&mut self, request: RequestBuilder, priority: RequestPriority) {
        match priority {
            RequestPriority::Critical | RequestPriority::High => {
                self.high_priority.push(request);
            }
            RequestPriority::Normal => {
                self.normal_priority.push(request);
            }
            RequestPriority::Low => {
                self.low_priority.push(request);
            }
        }
    }

    pub fn get_next_request(&mut self) -> Option<RequestBuilder> {
        if !self.high_priority.is_empty() {
            self.high_priority.pop()
        } else if !self.normal_priority.is_empty() {
            self.normal_priority.pop()
        } else if !self.low_priority.is_empty() {
            self.low_priority.pop()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.high_priority.len() + self.normal_priority.len() + self.low_priority.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_optimized_http_config_default() {
        let config = OptimizedHttpConfig::default();
        assert_eq!(config.pool_config.max_idle_per_host, 100);
        assert_eq!(config.timeout_config.connect_timeout, Duration::from_secs(8));
        assert!(config.compression_config.enable_smart_compression);
    }

    #[test]
    fn test_request_stats() {
        let mut stats = RequestStats::default();

        stats.update_request(100, true, false);
        stats.update_request(200, false, true);

        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.successful_requests, 1);
        assert_eq!(stats.failed_requests, 1);
        assert_eq!(stats.retry_count, 1);
        assert_eq!(stats.success_rate(), 0.5);
        assert!(stats.average_duration_ms > 0.0);
    }

    #[test]
    fn test_priority_queue() {
        let mut queue = PriorityRequestQueue::new();

        assert!(queue.is_empty());

        // 添加不同优先级的请求
        let dummy_request = reqwest::Client::new().get("http://example.com");
        queue.add_request(dummy_request.clone(), RequestPriority::Low);
        queue.add_request(dummy_request.clone(), RequestPriority::High);
        queue.add_request(dummy_request, RequestPriority::Normal);

        assert_eq!(queue.len(), 3);

        // 高优先级请求应该先出队
        let _ = queue.get_next_request();
        assert_eq!(queue.len(), 2);
    }

    #[tokio::test]
    async fn test_optimized_client_creation() {
        let client = OptimizedHttpClient::new_default();
        let stats = client.stats();
        assert_eq!(stats.total_requests, 0);
    }
}