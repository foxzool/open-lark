//! 性能基准测试工具
//!
//! 用于测量和比较open-lark SDK的性能指标，包括：
//! - HTTP请求性能测试
//! - 缓存性能测试
//! - 内存使用测试
//! - 序列化性能测试
//! - 并发性能测试

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, warn, error};

/// 基准测试配置
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// 测试持续时间
    pub duration: Duration,
    /// 并发数量
    pub concurrent_requests: usize,
    /// 测试数据大小
    pub data_size_kb: usize,
    /// 重复次数
    pub iterations: usize,
    /// 是否启用预热
    pub enable_warmup: bool,
    /// 预热时长
    pub warmup_duration: Duration,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(30),
            concurrent_requests: 10,
            data_size_kb: 10,
            iterations: 1000,
            enable_warmup: true,
            warmup_duration: Duration::from_secs(5),
        }
    }
}

/// 基准测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// 测试名称
    pub test_name: String,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 总耗时（毫秒）
    pub total_duration_ms: u64,
    /// 平均响应时间（毫秒）
    pub average_response_time_ms: f64,
    /// 最小响应时间（毫秒）
    pub min_response_time_ms: u64,
    /// 最大响应时间（毫秒）
    pub max_response_time_ms: u64,
    /// P95响应时间（毫秒）
    pub p95_response_time_ms: u64,
    /// P99响应时间（毫秒）
    pub p99_response_time_ms: u64,
    /// 吞吐量（请求/秒）
    pub throughput_rps: f64,
    /// 错误率
    pub error_rate: f64,
    /// 内存使用峰值（MB）
    pub peak_memory_mb: f64,
    /// CPU使用率
    pub cpu_usage_percent: f64,
}

impl BenchmarkResult {
    /// 创建新的基准测试结果
    pub fn new(test_name: String) -> Self {
        Self {
            test_name,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_duration_ms: 0,
            average_response_time_ms: 0.0,
            min_response_time_ms: u64::MAX,
            max_response_time_ms: 0,
            p95_response_time_ms: 0,
            p99_response_time_ms: 0,
            throughput_rps: 0.0,
            error_rate: 0.0,
            peak_memory_mb: 0.0,
            cpu_usage_percent: 0.0,
        }
    }

    /// 计算统计数据
    pub fn calculate_statistics(&mut self, response_times: &mut Vec<u64>) {
        if response_times.is_empty() {
            return;
        }

        response_times.sort_unstable();

        self.min_response_time_ms = response_times[0];
        self.max_response_time_ms = response_times[response_times.len() - 1];

        let sum: u64 = response_times.iter().sum();
        self.average_response_time_ms = sum as f64 / response_times.len() as f64;

        // 计算百分位数
        let p95_index = (response_times.len() as f64 * 0.95) as usize;
        let p99_index = (response_times.len() as f64 * 0.99) as usize;

        self.p95_response_time_ms = response_times[p95_index.min(response_times.len() - 1)];
        self.p99_response_time_ms = response_times[p99_index.min(response_times.len() - 1)];

        // 计算吞吐量和错误率
        if self.total_duration_ms > 0 {
            self.throughput_rps = self.total_requests as f64 / (self.total_duration_ms as f64 / 1000.0);
        }

        if self.total_requests > 0 {
            self.error_rate = self.failed_requests as f64 / self.total_requests as f64;
        }
    }

    /// 打印结果
    pub fn print_results(&self) {
        println!("=== {} 基准测试结果 ===", self.test_name);
        println!("总请求数: {}", self.total_requests);
        println!("成功请求: {}", self.successful_requests);
        println!("失败请求: {}", self.failed_requests);
        println!("总耗时: {}ms", self.total_duration_ms);
        println!("平均响应时间: {:.2}ms", self.average_response_time_ms);
        println!("最小响应时间: {}ms", self.min_response_time_ms);
        println!("最大响应时间: {}ms", self.max_response_time_ms);
        println!("P95响应时间: {}ms", self.p95_response_time_ms);
        println!("P99响应时间: {}ms", self.p99_response_time_ms);
        println!("吞吐量: {:.2} 请求/秒", self.throughput_rps);
        println!("错误率: {:.2}%", self.error_rate * 100.0);
        println!("峰值内存使用: {:.2} MB", self.peak_memory_mb);
        println!("CPU使用率: {:.1}%", self.cpu_usage_percent);
        println!();
    }
}

/// 性能基准测试套件
pub struct PerformanceBenchmark {
    config: BenchmarkConfig,
}

impl PerformanceBenchmark {
    /// 创建新的基准测试套件
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn new_default() -> Self {
        Self::new(BenchmarkConfig::default())
    }

    /// 运行所有基准测试
    pub async fn run_all_benchmarks(&self) -> Vec<BenchmarkResult> {
        let mut results = Vec::new();

        info!("开始运行性能基准测试...");

        // HTTP请求性能测试
        results.push(self.benchmark_http_performance().await);

        // 缓存性能测试
        results.push(self.benchmark_cache_performance().await);

        // 内存使用测试
        results.push(self.benchmark_memory_usage().await);

        // 序列化性能测试
        results.push(self.benchmark_serialization_performance().await);

        // 并发性能测试
        results.push(self.benchmark_concurrent_performance().await);

        // 打印总结
        self.print_summary(&results);

        results
    }

    /// HTTP请求性能测试
    pub async fn benchmark_http_performance(&self) -> BenchmarkResult {
        let test_name = "HTTP请求性能".to_string();
        let mut result = BenchmarkResult::new(test_name.clone());
        let mut response_times = Vec::new();

        info!("开始HTTP请求性能测试...");

        // 预热
        if self.config.enable_warmup {
            info!("预热中...");
            self.warmup_http_requests().await;
        }

        let start_time = Instant::now();
        let end_time = start_time + self.config.duration;

        while Instant::now() < end_time {
            let request_start = Instant::now();

            // 模拟HTTP请求
            let _ = self.simulate_http_request().await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            if response_time < 5000 { // 假设5秒内为成功
                result.successful_requests += 1;
            } else {
                result.failed_requests += 1;
            }

            // 控制请求频率
            sleep(Duration::from_millis(10)).await;
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
        result.calculate_statistics(&mut response_times);
        result.print_results();

        result
    }

    /// 缓存性能测试
    pub async fn benchmark_cache_performance(&self) -> BenchmarkResult {
        let test_name = "缓存性能".to_string();
        let mut result = BenchmarkResult::new(test_name.clone());
        let mut response_times = Vec::new();

        info!("开始缓存性能测试...");

        // 使用优化的缓存
        let cache = crate::core::performance_optimized_cache::PerformanceCache::new_default();
        let keys: Vec<String> = (0..1000)
            .map(|i| format!("benchmark_key_{}", i))
            .collect();

        // 预热缓存
        if self.config.enable_warmup {
            info!("预热缓存中...");
            cache.warm_up(keys.clone().into_iter().map(|k| async move {
                Some((k, format!("cached_value_for_{}", k)))
            })).await;
        }

        let start_time = Instant::now();
        let end_time = start_time + self.config.duration;

        while Instant::now() < end_time {
            let request_start = Instant::now();

            // 缓存操作
            let key = &keys[result.total_requests as usize % keys.len()];
            let _ = cache.get(key);

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            result.successful_requests += 1; // 缓存操作总是成功的

            // 控制操作频率
            sleep(Duration::from_micros(100)).await;
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
        result.calculate_statistics(&mut response_times);
        result.print_results();

        result
    }

    /// 内存使用测试
    pub async fn benchmark_memory_usage(&self) -> BenchmarkResult {
        let test_name = "内存使用".to_string();
        let mut result = BenchmarkResult::new(test_name.clone());
        let mut response_times = Vec::new();

        info!("开始内存使用测试...");

        let start_time = Instant::now();
        let end_time = start_time + self.config.duration;

        while Instant::now() < end_time {
            let request_start = Instant::now();

            // 模拟内存密集操作
            let _ = self.simulate_memory_usage(self.config.data_size_kb * 1024).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            result.successful_requests += 1;

            // 控制操作频率
            sleep(Duration::from_millis(50)).await;
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
        result.peak_memory_mb = self.config.data_size_kb as f64 / 1024.0; // 简化估算
        result.calculate_statistics(&mut response_times);
        result.print_results();

        result
    }

    /// 序列化性能测试
    pub async fn benchmark_serialization_performance(&self) -> BenchmarkResult {
        let test_name = "序列化性能".to_string();
        let mut result = BenchmarkResult::new(test_name.clone());
        let mut response_times = Vec::new();

        info!("开始序列化性能测试...");

        let start_time = Instant::now();
        let end_time = start_time + self.config.duration;

        while Instant::now() < end_time {
            let request_start = Instant::now();

            // 模拟序列化操作
            let _ = self.simulate_serialization(self.config.data_size_kb * 1024).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            result.successful_requests += 1;

            // 控制操作频率
            sleep(Duration::from_millis(20)).await;
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
        result.calculate_statistics(&mut response_times);
        result.print_results();

        result
    }

    /// 并发性能测试
    pub async fn benchmark_concurrent_performance(&self) -> BenchmarkResult {
        let test_name = "并发性能".to_string();
        let mut result = BenchmarkResult::new(test_name.clone());
        let mut response_times = Vec::new();

        info!("开始并发性能测试...");

        let start_time = Instant::now();
        let end_time = start_time + self.config.duration;

        while Instant::now() < end_time {
            let request_start = Instant::now();

            // 并发请求
            let tasks: Vec<_> = (0..self.config.concurrent_requests)
                .map(|_| async {
                    let _ = self.simulate_http_request().await;
                })
                .collect();

            futures_util::future::join_all(tasks).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += self.config.concurrent_requests as u64;
            result.successful_requests += self.config.concurrent_requests as u64;

            // 控制测试批次
            sleep(Duration::from_millis(100)).await;
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
        result.calculate_statistics(&mut response_times);
        result.print_results();

        result
    }

    /// 打印测试总结
    fn print_summary(&self, results: &[BenchmarkResult]) {
        println!("=== 性能基准测试总结 ===");
        println!("测试项目数量: {}", results.len());
        println!();

        for result in results {
            println!("{}: 吞吐量 {:.2} RPS, 平均响应时间 {:.2}ms, 错误率 {:.2}%",
                     result.test_name,
                     result.throughput_rps,
                     result.average_response_time_ms,
                     result.error_rate * 100.0);
        }

        // 计算平均值
        let avg_throughput = results.iter().map(|r| r.throughput_rps).sum::<f64>() / results.len() as f64;
        let avg_response_time = results.iter().map(|r| r.average_response_time_ms).sum::<f64>() / results.len() as f64;
        let avg_error_rate = results.iter().map(|r| r.error_rate).sum::<f64>() / results.len() as f64;

        println!("平均性能指标:");
        println!("  平均吞吐量: {:.2} RPS", avg_throughput);
        println!("  平均响应时间: {:.2} ms", avg_response_time);
        println!("  平均错误率: {:.2}%", avg_error_rate * 100.0);
        println!();
    }

    // 辅助方法

    async fn warmup_http_requests(&self) {
        for _ in 0..100 {
            let _ = self.simulate_http_request().await;
            sleep(Duration::from_millis(10)).await;
        }
    }

    async fn simulate_http_request(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 模拟HTTP请求延迟
        sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    async fn simulate_memory_usage(&self, size_bytes: usize) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // 模拟内存分配
        let data = vec![0u8; size_bytes];
        sleep(Duration::from_micros(size_bytes as u64 / 100)).await;
        Ok(data)
    }

    async fn simulate_serialization(&self, size_bytes: usize) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 模拟序列化操作
        let data = "x".repeat(size_bytes);
        let _ = serde_json::to_string(&data)?;
        sleep(Duration::from_micros(size_bytes as u64 / 200)).await;
        Ok(data)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = BenchmarkConfig {
        duration: Duration::from_secs(30),
        concurrent_requests: 50,
        data_size_kb: 50,
        iterations: 1000,
        enable_warmup: true,
        warmup_duration: Duration::from_secs(5),
    };

    let benchmark = PerformanceBenchmark::new(config);
    let results = benchmark.run_all_benchmarks().await;

    // 输出JSON格式结果
    println!("=== JSON格式结果 ===");
    println!("{}", serde_json::to_string_pretty(&results)?);

    Ok(())
}