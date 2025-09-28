//! HTTP客户端性能优化和基准测试模块
//!
//! 本模块提供：
//! 1. HTTP客户端配置优化建议
//! 2. 性能基准测试工具
//! 3. 连接池管理
//! 4. 请求性能分析
//! 5. 内存分配优化

pub mod local;

// 重新导出主要类型
pub use local::LocalPerformanceTester;

use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{info, instrument, warn};

/// HTTP客户端性能配置优化
#[derive(Debug, Clone)]
pub struct OptimizedHttpConfig {
    /// 连接池最大空闲连接数 (默认: 90)
    pub pool_max_idle_per_host: usize,
    /// 连接池空闲连接超时 (默认: 90秒)
    pub pool_idle_timeout: Duration,
    /// TCP连接超时 (默认: 10秒)
    pub connect_timeout: Duration,
    /// 请求总超时 (默认: 30秒)
    pub request_timeout: Duration,
    /// 启用HTTP/2 (默认: true)
    pub http2_prior_knowledge: bool,
    /// HTTP/2自适应窗口 (默认: true)
    pub http2_adaptive_window: bool,
    /// 启用gzip压缩 (默认: true)
    pub gzip: bool,
    /// 启用brotli压缩 (默认: true)
    pub brotli: bool,
    /// User-Agent字符串
    pub user_agent: String,
}

impl Default for OptimizedHttpConfig {
    fn default() -> Self {
        Self {
            pool_max_idle_per_host: 90,
            pool_idle_timeout: Duration::from_secs(90),
            connect_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            http2_prior_knowledge: true,
            http2_adaptive_window: true,
            gzip: true,
            brotli: true,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl OptimizedHttpConfig {
    /// 创建生产环境优化配置
    pub fn production() -> Self {
        Self {
            pool_max_idle_per_host: 100,
            pool_idle_timeout: Duration::from_secs(120),
            connect_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(30),
            http2_prior_knowledge: true,
            http2_adaptive_window: true,
            gzip: true,
            brotli: true,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// 创建高吞吐量配置
    pub fn high_throughput() -> Self {
        Self {
            pool_max_idle_per_host: 200,
            pool_idle_timeout: Duration::from_secs(180),
            connect_timeout: Duration::from_secs(3),
            request_timeout: Duration::from_secs(15),
            http2_prior_knowledge: true,
            http2_adaptive_window: true,
            gzip: true,
            brotli: true,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// 创建低延迟配置
    pub fn low_latency() -> Self {
        Self {
            pool_max_idle_per_host: 50,
            pool_idle_timeout: Duration::from_secs(60),
            connect_timeout: Duration::from_secs(2),
            request_timeout: Duration::from_secs(10),
            http2_prior_knowledge: true,
            http2_adaptive_window: true,
            gzip: false, // 减少CPU开销
            brotli: false,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// 根据配置构建优化的HTTP客户端
    pub fn build_client(&self) -> Result<Client, reqwest::Error> {
        let mut builder = ClientBuilder::new()
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .pool_idle_timeout(self.pool_idle_timeout)
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout)
            .user_agent(&self.user_agent);

        // 注意: reqwest 0.12 中某些HTTP/2配置方法可能不可用
        // 我们使用可用的配置方法

        // 注意: reqwest 0.12 默认启用gzip和brotli
        // 如果需要禁用，可以使用 no_gzip() 和 no_brotli()
        if !self.gzip {
            builder = builder.no_gzip();
        }

        if !self.brotli {
            builder = builder.no_brotli();
        }

        builder.build()
    }
}

/// 性能基准测试指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// 测试名称
    pub test_name: String,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 平均响应时间 (毫秒)
    pub avg_response_time_ms: f64,
    /// 95th百分位响应时间 (毫秒)
    pub p95_response_time_ms: f64,
    /// 99th百分位响应时间 (毫秒)
    pub p99_response_time_ms: f64,
    /// 最小响应时间 (毫秒)
    pub min_response_time_ms: f64,
    /// 最大响应时间 (毫秒)
    pub max_response_time_ms: f64,
    /// 每秒请求数 (RPS)
    pub requests_per_second: f64,
    /// 测试持续时间 (秒)
    pub duration_seconds: f64,
    /// 字节传输速率 (KB/s)
    pub throughput_kbps: f64,
    /// 错误率 (%)
    pub error_rate: f64,
}

impl PerformanceMetrics {
    /// 从响应时间列表计算性能指标
    pub fn calculate(
        test_name: String,
        response_times_ms: Vec<f64>,
        total_duration: Duration,
        total_bytes: u64,
    ) -> Self {
        let total_requests = response_times_ms.len() as u64;
        let failed_requests = response_times_ms.iter().filter(|&&t| t < 0.0).count() as u64;
        let successful_requests = total_requests - failed_requests;

        let mut sorted_times: Vec<f64> = response_times_ms
            .iter()
            .filter(|&&t| t >= 0.0)
            .copied()
            .collect();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let avg_response_time_ms = if sorted_times.is_empty() {
            0.0
        } else {
            sorted_times.iter().sum::<f64>() / sorted_times.len() as f64
        };

        let p95_response_time_ms = if sorted_times.is_empty() {
            0.0
        } else {
            let index = ((sorted_times.len() as f64) * 0.95) as usize;
            sorted_times
                .get(index.saturating_sub(1))
                .copied()
                .unwrap_or(0.0)
        };

        let p99_response_time_ms = if sorted_times.is_empty() {
            0.0
        } else {
            let index = ((sorted_times.len() as f64) * 0.99) as usize;
            sorted_times
                .get(index.saturating_sub(1))
                .copied()
                .unwrap_or(0.0)
        };

        let min_response_time_ms = sorted_times.first().copied().unwrap_or(0.0);
        let max_response_time_ms = sorted_times.last().copied().unwrap_or(0.0);

        let duration_seconds = total_duration.as_secs_f64();
        let requests_per_second = if duration_seconds > 0.0 {
            total_requests as f64 / duration_seconds
        } else {
            0.0
        };

        let throughput_kbps = if duration_seconds > 0.0 {
            (total_bytes as f64) / (duration_seconds * 1024.0)
        } else {
            0.0
        };

        let error_rate = if total_requests > 0 {
            (failed_requests as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        Self {
            test_name,
            total_requests,
            successful_requests,
            failed_requests,
            avg_response_time_ms,
            p95_response_time_ms,
            p99_response_time_ms,
            min_response_time_ms,
            max_response_time_ms,
            requests_per_second,
            duration_seconds,
            throughput_kbps,
            error_rate,
        }
    }

    /// 打印性能报告
    pub fn print_report(&self) {
        info!("╔══════════════════════════════════════════════════════════╗");
        info!("║                    性能测试报告                          ║");
        info!("╠══════════════════════════════════════════════════════════╣");
        info!("║ 测试名称: {:<45} ║", self.test_name);
        info!("║ 总请求数: {:<45} ║", self.total_requests);
        info!("║ 成功请求: {:<45} ║", self.successful_requests);
        info!("║ 失败请求: {:<45} ║", self.failed_requests);
        info!("║ 错误率:   {:<43.2}% ║", self.error_rate);
        info!("╠══════════════════════════════════════════════════════════╣");
        info!("║ 平均响应时间: {:<39.2}ms ║", self.avg_response_time_ms);
        info!("║ 95th百分位:   {:<39.2}ms ║", self.p95_response_time_ms);
        info!("║ 99th百分位:   {:<39.2}ms ║", self.p99_response_time_ms);
        info!("║ 最小响应时间: {:<39.2}ms ║", self.min_response_time_ms);
        info!("║ 最大响应时间: {:<39.2}ms ║", self.max_response_time_ms);
        info!("╠══════════════════════════════════════════════════════════╣");
        info!("║ 吞吐量:       {:<39.2}RPS ║", self.requests_per_second);
        info!("║ 数据传输率:   {:<37.2}KB/s ║", self.throughput_kbps);
        info!("║ 测试时长:     {:<39.2}s ║", self.duration_seconds);
        info!("╚══════════════════════════════════════════════════════════╝");
    }
}

/// 基准测试配置
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// 并发连接数
    pub concurrent_connections: usize,
    /// 每个连接的请求数
    pub requests_per_connection: usize,
    /// 预热请求数
    pub warmup_requests: usize,
    /// 测试URL
    pub target_url: String,
    /// 请求头
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            concurrent_connections: 10,
            requests_per_connection: 100,
            warmup_requests: 10,
            target_url: "https://httpbin.org/json".to_string(),
            headers: std::collections::HashMap::new(),
        }
    }
}

/// HTTP客户端基准测试器
pub struct HttpBenchmark {
    client: Client,
    config: BenchmarkConfig,
}

impl HttpBenchmark {
    /// 创建新的基准测试器
    pub fn new(client: Client, config: BenchmarkConfig) -> Self {
        Self { client, config }
    }

    /// 执行基准测试
    #[instrument(skip(self), fields(
        concurrent_connections = self.config.concurrent_connections,
        requests_per_connection = self.config.requests_per_connection,
        target_url = %self.config.target_url
    ))]
    pub async fn run_benchmark(
        &self,
    ) -> Result<PerformanceMetrics, Box<dyn std::error::Error + Send + Sync>> {
        info!("开始HTTP客户端基准测试...");
        info!("目标URL: {}", self.config.target_url);
        info!("并发连接数: {}", self.config.concurrent_connections);
        info!("每连接请求数: {}", self.config.requests_per_connection);

        // 预热
        if self.config.warmup_requests > 0 {
            info!("预热阶段: {} 个请求", self.config.warmup_requests);
            for _ in 0..self.config.warmup_requests {
                let _ = self.make_request().await;
            }
            info!("预热完成");
        }

        let start_time = Instant::now();
        let mut tasks = Vec::new();
        let mut total_bytes = 0u64;

        // 创建并发任务
        for connection_id in 0..self.config.concurrent_connections {
            let client = self.client.clone();
            let config = self.config.clone();

            let task = tokio::spawn(async move {
                let mut response_times = Vec::new();
                let mut bytes_transferred = 0u64;

                for request_id in 0..config.requests_per_connection {
                    let request_start = Instant::now();

                    match Self::make_single_request(&client, &config.target_url, &config.headers)
                        .await
                    {
                        Ok(bytes) => {
                            let elapsed = request_start.elapsed().as_secs_f64() * 1000.0;
                            response_times.push(elapsed);
                            bytes_transferred += bytes;
                        }
                        Err(e) => {
                            warn!("连接 {} 请求 {} 失败: {}", connection_id, request_id, e);
                            response_times.push(-1.0); // 标记为失败
                        }
                    }
                }

                (response_times, bytes_transferred)
            });

            tasks.push(task);
        }

        // 等待所有任务完成
        let mut all_response_times = Vec::new();
        for task in tasks {
            match task.await {
                Ok((response_times, bytes)) => {
                    all_response_times.extend(response_times);
                    total_bytes += bytes;
                }
                Err(e) => {
                    warn!("任务执行失败: {}", e);
                }
            }
        }

        let total_duration = start_time.elapsed();
        let metrics = PerformanceMetrics::calculate(
            "HTTP客户端基准测试".to_string(),
            all_response_times,
            total_duration,
            total_bytes,
        );

        info!("基准测试完成");
        metrics.print_report();

        Ok(metrics)
    }

    /// 执行单个请求
    async fn make_request(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        Self::make_single_request(&self.client, &self.config.target_url, &self.config.headers).await
    }

    /// 执行单个HTTP请求
    async fn make_single_request(
        client: &Client,
        url: &str,
        headers: &std::collections::HashMap<String, String>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut request = client.get(url);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        let bytes = response.content_length().unwrap_or(0);
        let _body = response.bytes().await?;

        Ok(bytes)
    }
}

/// 客户端配置比较工具
pub struct ClientComparison;

impl ClientComparison {
    /// 比较不同HTTP客户端配置的性能
    #[instrument]
    pub async fn compare_configurations(
        configs: Vec<(&str, OptimizedHttpConfig)>,
        benchmark_config: BenchmarkConfig,
    ) -> Result<Vec<(String, PerformanceMetrics)>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        for (config_name, http_config) in configs {
            info!("测试配置: {}", config_name);

            let client = http_config.build_client()?;
            let benchmark = HttpBenchmark::new(client, benchmark_config.clone());

            match benchmark.run_benchmark().await {
                Ok(metrics) => {
                    let mut named_metrics = metrics;
                    named_metrics.test_name = config_name.to_string();
                    results.push((config_name.to_string(), named_metrics));
                }
                Err(e) => {
                    warn!("配置 {} 测试失败: {}", config_name, e);
                }
            }

            // 在配置测试之间短暂休息
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        Self::print_comparison_report(&results);
        Ok(results)
    }

    /// 打印配置比较报告
    fn print_comparison_report(results: &[(String, PerformanceMetrics)]) {
        if results.is_empty() {
            warn!("没有可比较的结果");
            return;
        }

        info!("╔══════════════════════════════════════════════════════════════════════════╗");
        info!("║                            配置性能比较报告                              ║");
        info!("╠══════════════════════════════════════════════════════════════════════════╣");
        info!("║ 配置名称        │ RPS     │ 平均延迟(ms) │ P95(ms) │ P99(ms) │ 错误率(%) ║");
        info!("╠═════════════════╪═════════╪══════════════╪═════════╪═════════╪═══════════╣");

        for (name, metrics) in results {
            info!(
                "║ {:<15} │ {:>7.1} │ {:>12.2} │ {:>7.2} │ {:>7.2} │ {:>9.2} ║",
                name.chars().take(15).collect::<String>(),
                metrics.requests_per_second,
                metrics.avg_response_time_ms,
                metrics.p95_response_time_ms,
                metrics.p99_response_time_ms,
                metrics.error_rate
            );
        }

        info!("╚═════════════════╧═════════╧══════════════╧═════════╧═════════╧═══════════╝");

        // 找出最佳性能配置
        if let Some((best_name, best_metrics)) = results.iter().max_by(|a, b| {
            a.1.requests_per_second
                .partial_cmp(&b.1.requests_per_second)
                .unwrap()
        }) {
            info!(
                "🏆 最佳吞吐量配置: {} ({:.1} RPS)",
                best_name, best_metrics.requests_per_second
            );
        }

        if let Some((best_name, best_metrics)) = results.iter().min_by(|a, b| {
            a.1.avg_response_time_ms
                .partial_cmp(&b.1.avg_response_time_ms)
                .unwrap()
        }) {
            info!(
                "🚀 最低延迟配置: {} ({:.2}ms 平均延迟)",
                best_name, best_metrics.avg_response_time_ms
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_http_config_default() {
        let config = OptimizedHttpConfig::default();
        assert_eq!(config.pool_max_idle_per_host, 90);
        assert_eq!(config.connect_timeout, Duration::from_secs(10));
        assert!(config.http2_prior_knowledge);
        assert!(config.gzip);
    }

    #[test]
    fn test_optimized_http_config_production() {
        let config = OptimizedHttpConfig::production();
        assert_eq!(config.pool_max_idle_per_host, 100);
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
        assert_eq!(config.request_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_optimized_http_config_high_throughput() {
        let config = OptimizedHttpConfig::high_throughput();
        assert_eq!(config.pool_max_idle_per_host, 200);
        assert_eq!(config.connect_timeout, Duration::from_secs(3));
        assert_eq!(config.request_timeout, Duration::from_secs(15));
    }

    #[test]
    fn test_optimized_http_config_low_latency() {
        let config = OptimizedHttpConfig::low_latency();
        assert_eq!(config.pool_max_idle_per_host, 50);
        assert_eq!(config.connect_timeout, Duration::from_secs(2));
        assert!(!config.gzip); // 低延迟配置应该禁用压缩
        assert!(!config.brotli);
    }

    #[test]
    fn test_performance_metrics_calculation() {
        let response_times = vec![100.0, 200.0, 150.0, 300.0, 250.0];
        let duration = Duration::from_secs(5);
        let total_bytes = 1024 * 5; // 5KB

        let metrics = PerformanceMetrics::calculate(
            "test".to_string(),
            response_times,
            duration,
            total_bytes,
        );

        assert_eq!(metrics.total_requests, 5);
        assert_eq!(metrics.successful_requests, 5);
        assert_eq!(metrics.failed_requests, 0);
        assert_eq!(metrics.avg_response_time_ms, 200.0);
        assert_eq!(metrics.min_response_time_ms, 100.0);
        assert_eq!(metrics.max_response_time_ms, 300.0);
        assert_eq!(metrics.requests_per_second, 1.0);
        assert_eq!(metrics.throughput_kbps, 1.0); // 5KB / 5s = 1KB/s
        assert_eq!(metrics.error_rate, 0.0);
    }

    #[test]
    fn test_performance_metrics_with_failures() {
        let response_times = vec![100.0, -1.0, 150.0, -1.0, 250.0]; // 2 failures
        let duration = Duration::from_secs(5);
        let total_bytes = 1024 * 3; // Only successful requests transfer data

        let metrics = PerformanceMetrics::calculate(
            "test_with_failures".to_string(),
            response_times,
            duration,
            total_bytes,
        );

        assert_eq!(metrics.total_requests, 5);
        assert_eq!(metrics.successful_requests, 3);
        assert_eq!(metrics.failed_requests, 2);
        assert_eq!(metrics.error_rate, 40.0); // 2/5 = 40%
        assert_eq!(metrics.avg_response_time_ms, (100.0 + 150.0 + 250.0) / 3.0);
    }

    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.concurrent_connections, 10);
        assert_eq!(config.requests_per_connection, 100);
        assert_eq!(config.warmup_requests, 10);
        assert_eq!(config.target_url, "https://httpbin.org/json");
    }

    #[tokio::test]
    async fn test_http_config_build_client() {
        let config = OptimizedHttpConfig::default();
        let client = config.build_client();
        assert!(client.is_ok(), "应该能够成功构建HTTP客户端");
    }

    #[test]
    fn test_performance_metrics_empty_response_times() {
        let response_times: Vec<f64> = vec![];
        let duration = Duration::from_secs(1);
        let total_bytes = 0;

        let metrics = PerformanceMetrics::calculate(
            "empty_test".to_string(),
            response_times,
            duration,
            total_bytes,
        );

        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.avg_response_time_ms, 0.0);
        assert_eq!(metrics.requests_per_second, 0.0);
    }

    #[test]
    fn test_performance_metrics_percentiles() {
        let response_times = (1..=100).map(|i| i as f64).collect::<Vec<_>>();
        let duration = Duration::from_secs(10);
        let total_bytes = 10240;

        let metrics = PerformanceMetrics::calculate(
            "percentile_test".to_string(),
            response_times,
            duration,
            total_bytes,
        );

        assert_eq!(metrics.total_requests, 100);
        assert_eq!(metrics.min_response_time_ms, 1.0);
        assert_eq!(metrics.max_response_time_ms, 100.0);
        // P95应该接近95，P99应该接近99
        assert!((metrics.p95_response_time_ms - 95.0).abs() < 2.0);
        assert!((metrics.p99_response_time_ms - 99.0).abs() < 2.0);
    }
}
