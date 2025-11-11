//! 本地性能测试模块
//!
//! 提供不依赖外部网络的性能测试功能

use crate::performance::{OptimizedHttpConfig, PerformanceMetrics};
use std::time::Instant;
use tracing::info;

/// 本地HTTP客户端配置测试器
pub struct LocalPerformanceTester;

impl LocalPerformanceTester {
    /// 测试HTTP客户端构建性能
    pub fn test_client_creation_performance() -> PerformanceMetrics {
        info!("🔧 测试HTTP客户端创建性能...");

        let configs = vec![
            ("默认配置", OptimizedHttpConfig::default()),
            ("生产环境", OptimizedHttpConfig::production()),
            ("高吞吐量", OptimizedHttpConfig::high_throughput()),
            ("低延迟", OptimizedHttpConfig::low_latency()),
        ];

        let mut all_times = Vec::new();
        let start_time = Instant::now();
        let mut _total_builds = 0;

        for (name, config) in configs {
            info!("测试配置: {}", name);

            // 测试每个配置的客户端创建时间
            for _ in 0..10 {
                let build_start = Instant::now();

                match config.build_client() {
                    Ok(_client) => {
                        let build_time = build_start.elapsed().as_secs_f64() * 1000.0;
                        all_times.push(build_time);
                        _total_builds += 1;
                    }
                    Err(_) => {
                        all_times.push(-1.0); // 标记失败
                    }
                }
            }
        }

        let total_duration = start_time.elapsed();

        PerformanceMetrics::calculate(
            "HTTP客户端创建性能".to_string(),
            all_times,
            total_duration,
            0, // 没有字节传输
        )
    }

    /// 测试配置对象创建性能
    pub fn test_config_creation_performance() -> PerformanceMetrics {
        info!("⚙️ 测试配置对象创建性能...");

        let iterations = 1000;
        let mut response_times = Vec::new();
        let start_time = Instant::now();

        for i in 0..iterations {
            let config_start = Instant::now();

            // 创建不同类型的配置
            match i % 4 {
                0 => {
                    let _ = OptimizedHttpConfig::default();
                }
                1 => {
                    let _ = OptimizedHttpConfig::production();
                }
                2 => {
                    let _ = OptimizedHttpConfig::high_throughput();
                }
                3 => {
                    let _ = OptimizedHttpConfig::low_latency();
                }
                _ => unreachable!(),
            }

            let elapsed = config_start.elapsed().as_secs_f64() * 1000.0;
            response_times.push(elapsed);
        }

        let total_duration = start_time.elapsed();

        PerformanceMetrics::calculate(
            "配置对象创建性能".to_string(),
            response_times,
            total_duration,
            0,
        )
    }

    /// 运行综合本地性能测试
    pub fn run_comprehensive_local_test() -> Vec<PerformanceMetrics> {
        info!("🚀 开始综合本地性能测试...");

        let mut results = Vec::new();

        // 测试1: 配置对象创建
        let config_metrics = Self::test_config_creation_performance();
        config_metrics.print_report();
        results.push(config_metrics);

        // 测试2: HTTP客户端创建
        let client_metrics = Self::test_client_creation_performance();
        client_metrics.print_report();
        results.push(client_metrics);

        // 测试3: 内存配置验证
        let memory_metrics = Self::test_memory_efficiency();
        memory_metrics.print_report();
        results.push(memory_metrics);

        info!("✅ 综合本地性能测试完成");
        Self::print_summary(&results);

        results
    }

    /// 测试内存效率
    fn test_memory_efficiency() -> PerformanceMetrics {
        info!("💾 测试内存效率...");

        let iterations = 500;
        let mut response_times = Vec::new();
        let start_time = Instant::now();

        for _ in 0..iterations {
            let alloc_start = Instant::now();

            // 创建和释放配置对象
            let configs = vec![
                OptimizedHttpConfig::default(),
                OptimizedHttpConfig::production(),
                OptimizedHttpConfig::high_throughput(),
                OptimizedHttpConfig::low_latency(),
            ];

            // 模拟配置使用
            for config in configs {
                let _user_agent = config.user_agent.clone();
                let _timeout = config.connect_timeout;
                let _pool_size = config.pool_max_idle_per_host;
            }

            let elapsed = alloc_start.elapsed().as_secs_f64() * 1000.0;
            response_times.push(elapsed);
        }

        let total_duration = start_time.elapsed();

        PerformanceMetrics::calculate(
            "内存效率测试".to_string(),
            response_times,
            total_duration,
            0,
        )
    }

    /// 打印测试摘要
    fn print_summary(results: &[PerformanceMetrics]) {
        info!("╔══════════════════════════════════════════════════════════════════════════╗");
        info!("║                           📊 本地性能测试摘要                           ║");
        info!("╠══════════════════════════════════════════════════════════════════════════╣");

        for metrics in results {
            info!(
                "║ {:<20} │ {:>8.2}ms │ {:>10.1}/s │ {:>8.2}% ║",
                metrics.test_name.chars().take(20).collect::<String>(),
                metrics.avg_response_time_ms,
                metrics.requests_per_second,
                metrics.error_rate
            );
        }

        info!("╚══════════════════════════════════════════════════════════════════════════╝");

        // 性能建议
        info!("🎯 性能建议:");

        if let Some(config_metrics) = results.iter().find(|m| m.test_name.contains("配置对象"))
        {
            if config_metrics.avg_response_time_ms < 0.1 {
                info!("   ✅ 配置对象创建性能优秀 (<0.1ms)");
            } else if config_metrics.avg_response_time_ms < 1.0 {
                info!("   ⚠️  配置对象创建性能可接受 (<1ms)");
            } else {
                info!("   ❌ 配置对象创建性能需要优化 (>1ms)");
            }
        }

        if let Some(client_metrics) = results.iter().find(|m| m.test_name.contains("客户端")) {
            if client_metrics.avg_response_time_ms < 10.0 {
                info!("   ✅ HTTP客户端创建性能优秀 (<10ms)");
            } else if client_metrics.avg_response_time_ms < 50.0 {
                info!("   ⚠️  HTTP客户端创建性能可接受 (<50ms)");
            } else {
                info!("   ❌ HTTP客户端创建性能需要优化 (>50ms)");
            }
        }

        if let Some(memory_metrics) = results.iter().find(|m| m.test_name.contains("内存")) {
            if memory_metrics.avg_response_time_ms < 0.5 {
                info!("   ✅ 内存分配效率优秀 (<0.5ms)");
            } else if memory_metrics.avg_response_time_ms < 2.0 {
                info!("   ⚠️  内存分配效率可接受 (<2ms)");
            } else {
                info!("   ❌ 内存分配效率需要优化 (>2ms)");
            }
        }
    }

    /// 运行快速性能检查
    pub fn quick_performance_check() -> bool {
        info!("⚡ 运行快速性能检查...");

        let start = Instant::now();

        // 快速测试：创建各种配置
        let _default = OptimizedHttpConfig::default();
        let _production = OptimizedHttpConfig::production();
        let _high_throughput = OptimizedHttpConfig::high_throughput();
        let _low_latency = OptimizedHttpConfig::low_latency();

        let config_creation_time = start.elapsed();

        // 快速测试：构建客户端
        let client_start = Instant::now();
        let _client = OptimizedHttpConfig::production().build_client();
        let client_creation_time = client_start.elapsed();

        let total_time = start.elapsed();

        info!("配置创建时间: {:?}", config_creation_time);
        info!("客户端创建时间: {:?}", client_creation_time);
        info!("总计时间: {:?}", total_time);

        // 性能阈值检查
        let performance_ok = config_creation_time.as_millis() < 1
            && client_creation_time.as_millis() < 100
            && total_time.as_millis() < 150;

        if performance_ok {
            info!("✅ 性能检查通过");
        } else {
            info!("⚠️  性能检查未通过，可能需要优化");
        }

        performance_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation_performance() {
        let metrics = LocalPerformanceTester::test_config_creation_performance();

        // 配置创建应该非常快
        assert!(
            metrics.avg_response_time_ms < 1.0,
            "配置创建平均时间应该小于1ms，实际: {}ms",
            metrics.avg_response_time_ms
        );
        assert_eq!(metrics.error_rate, 0.0, "配置创建不应该有错误");
        assert!(
            metrics.requests_per_second > 1000.0,
            "配置创建速率应该大于1000/s，实际: {}/s",
            metrics.requests_per_second
        );
    }

    #[test]
    fn test_client_creation_performance() {
        let metrics = LocalPerformanceTester::test_client_creation_performance();

        // HTTP客户端创建应该合理快速
        assert!(
            metrics.avg_response_time_ms < 100.0,
            "客户端创建平均时间应该小于100ms，实际: {}ms",
            metrics.avg_response_time_ms
        );
        assert!(
            metrics.error_rate < 5.0,
            "客户端创建错误率应该小于5%，实际: {}%",
            metrics.error_rate
        );
    }

    #[test]
    fn test_quick_performance_check() {
        let result = LocalPerformanceTester::quick_performance_check();
        // 在正常系统上快速检查应该通过
        // 注意：在CI环境中可能会失败，这是正常的
        println!(
            "快速性能检查结果: {}",
            if result { "通过" } else { "未通过" }
        );
    }

    #[test]
    fn test_memory_efficiency() {
        let metrics = LocalPerformanceTester::test_memory_efficiency();

        // 内存分配应该高效
        assert!(
            metrics.avg_response_time_ms < 5.0,
            "内存分配平均时间应该小于5ms，实际: {}ms",
            metrics.avg_response_time_ms
        );
        assert_eq!(metrics.error_rate, 0.0, "内存分配不应该有错误");
    }

    #[test]
    fn test_comprehensive_local_test() {
        let results = LocalPerformanceTester::run_comprehensive_local_test();

        assert!(!results.is_empty(), "应该有测试结果");
        assert_eq!(results.len(), 3, "应该有3个测试结果");

        // 验证所有测试都成功运行
        for result in &results {
            assert!(
                result.total_requests > 0,
                "测试 {} 应该有请求",
                result.test_name
            );
        }
    }
}
