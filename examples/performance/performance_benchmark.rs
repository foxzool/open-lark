//! 性能基准测试示例
//!
//! 演示 open-lark SDK 的性能测试和监控功能

use serde::Serialize;
use std::time::{Duration, Instant};
use tracing::info;

/// 基准测试配置
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// 测试持续时间
    pub duration: Duration,
    /// 并发数量
    pub concurrent_requests: usize,
    /// 重复次数
    pub iterations: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(10),
            concurrent_requests: 10,
            iterations: 100,
        }
    }
}

/// 基准测试结果
#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkResult {
    /// 测试名称
    pub test_name: String,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 总耗时（毫秒）
    pub total_duration_ms: u64,
    /// 平均响应时间（毫秒）
    pub average_response_time_ms: f64,
    /// 吞吐量（请求/秒）
    pub throughput_rps: f64,
}

impl BenchmarkResult {
    /// 创建新的基准测试结果
    pub fn new(test_name: String) -> Self {
        Self {
            test_name,
            total_requests: 0,
            successful_requests: 0,
            total_duration_ms: 0,
            average_response_time_ms: 0.0,
            throughput_rps: 0.0,
        }
    }

    /// 计算统计数据
    pub fn calculate_statistics(&mut self, response_times: &mut Vec<u64>) {
        if response_times.is_empty() {
            return;
        }

        let sum: u64 = response_times.iter().sum();
        self.average_response_time_ms = sum as f64 / response_times.len() as f64;

        // 计算吞吐量
        if self.total_duration_ms > 0 {
            self.throughput_rps =
                self.total_requests as f64 / (self.total_duration_ms as f64 / 1000.0);
        }
    }

    /// 打印结果
    pub fn print_results(&self) {
        println!("=== {} 基准测试结果 ===", self.test_name);
        println!("总请求数: {}", self.total_requests);
        println!("成功请求: {}", self.successful_requests);
        println!("总耗时: {}ms", self.total_duration_ms);
        println!("平均响应时间: {:.2}ms", self.average_response_time_ms);
        println!("吞吐量: {:.2} 请求/秒", self.throughput_rps);
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

        let start_time = Instant::now();
        let end_time = start_time + self.config.duration;

        while Instant::now() < end_time {
            let request_start = Instant::now();

            // 模拟HTTP请求延迟
            tokio::time::sleep(Duration::from_millis(50)).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            result.successful_requests += 1;

            // 控制请求频率
            tokio::time::sleep(Duration::from_millis(10)).await;
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
            let _data: Vec<u8> = vec![0u8; 1024 * 10]; // 10KB
            tokio::time::sleep(Duration::from_micros(100)).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            result.successful_requests += 1;

            // 控制操作频率
            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
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
            let data = "x".repeat(100);
            let _json_str = serde_json::to_string(&data).unwrap_or_default();
            tokio::time::sleep(Duration::from_micros(200)).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += 1;
            result.successful_requests += 1;

            // 控制操作频率
            tokio::time::sleep(Duration::from_millis(15)).await;
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

            // 并发任务
            let tasks: Vec<_> = (0..self.config.concurrent_requests)
                .map(|_| async {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .collect();

            futures_util::future::join_all(tasks).await;

            let response_time = request_start.elapsed().as_millis() as u64;
            response_times.push(response_time);

            result.total_requests += self.config.concurrent_requests as u64;
            result.successful_requests += self.config.concurrent_requests as u64;

            // 控制测试批次
            tokio::time::sleep(Duration::from_millis(50)).await;
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
            println!(
                "{}: 吞吐量 {:.2} RPS, 平均响应时间 {:.2}ms",
                result.test_name, result.throughput_rps, result.average_response_time_ms
            );
        }

        // 计算平均值
        let avg_throughput =
            results.iter().map(|r| r.throughput_rps).sum::<f64>() / results.len() as f64;
        let avg_response_time = results
            .iter()
            .map(|r| r.average_response_time_ms)
            .sum::<f64>()
            / results.len() as f64;

        println!("\n平均性能指标:");
        println!("  平均吞吐量: {:.2} RPS", avg_throughput);
        println!("  平均响应时间: {:.2} ms", avg_response_time);
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = BenchmarkConfig {
        duration: Duration::from_secs(10),
        concurrent_requests: 20,
        iterations: 100,
    };

    let benchmark = PerformanceBenchmark::new(config);
    let results = benchmark.run_all_benchmarks().await;

    // 输出JSON格式结果
    println!("=== JSON格式结果 ===");
    println!("{}", serde_json::to_string_pretty(&results)?);

    println!("🎉 性能基准测试完成！");
    println!("✅ 所有测试已完成，查看上述结果了解性能指标");

    Ok(())
}
