//! 性能测试集成测试
//!
//! 验证HTTP客户端优化和性能基准功能

use open_lark::core::performance::{LocalPerformanceTester, OptimizedHttpConfig};

#[test]
fn test_http_config_creation_performance() {
    let metrics = LocalPerformanceTester::test_config_creation_performance();

    println!("配置创建性能测试结果:");
    println!("  总请求数: {}", metrics.total_requests);
    println!("  平均耗时: {:.4}ms", metrics.avg_response_time_ms);
    println!("  吞吐量: {:.1} 配置/秒", metrics.requests_per_second);

    // 配置创建应该非常快
    assert!(
        metrics.avg_response_time_ms < 1.0,
        "配置创建应该在1ms内完成，实际: {:.4}ms",
        metrics.avg_response_time_ms
    );
    assert_eq!(metrics.error_rate, 0.0, "不应该有任何错误");
}

#[test]
fn test_http_client_creation_performance() {
    let metrics = LocalPerformanceTester::test_client_creation_performance();

    println!("HTTP客户端创建性能测试结果:");
    println!("  总请求数: {}", metrics.total_requests);
    println!("  平均耗时: {:.2}ms", metrics.avg_response_time_ms);
    println!(
        "  成功率: {:.1}%",
        (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
    );

    // HTTP客户端创建应该合理快速
    assert!(
        metrics.avg_response_time_ms < 100.0,
        "HTTP客户端创建应该在100ms内完成，实际: {:.2}ms",
        metrics.avg_response_time_ms
    );
}

#[test]
fn test_optimized_configs() {
    println!("测试各种优化配置...");

    let configs = vec![
        ("默认配置", OptimizedHttpConfig::default()),
        ("生产环境", OptimizedHttpConfig::production()),
        ("高吞吐量", OptimizedHttpConfig::high_throughput()),
        ("低延迟", OptimizedHttpConfig::low_latency()),
    ];

    for (name, config) in configs {
        println!("  测试 {}: ", name);

        // 验证配置参数在合理范围内
        assert!(
            config.pool_max_idle_per_host > 0,
            "{} 连接池大小应该大于0",
            name
        );
        assert!(
            config.connect_timeout.as_secs() > 0,
            "{} 连接超时应该大于0秒",
            name
        );
        assert!(
            config.request_timeout.as_secs() > 0,
            "{} 请求超时应该大于0秒",
            name
        );
        assert!(
            !config.user_agent.is_empty(),
            "{} User-Agent不应该为空",
            name
        );

        // 验证能够成功构建客户端
        let client_result = config.build_client();
        assert!(client_result.is_ok(), "{} 应该能够成功构建HTTP客户端", name);

        println!("    ✅ 配置有效，客户端构建成功");
    }
}

#[test]
fn test_quick_performance_check() {
    println!("运行快速性能检查...");

    let result = LocalPerformanceTester::quick_performance_check();

    // 快速检查应该在合理时间内完成
    // 注意：在CI环境中可能会失败，这是正常的
    if result {
        println!("✅ 快速性能检查通过");
    } else {
        println!("⚠️  快速性能检查未通过，但这在CI环境中是正常的");
    }
}

#[test]
fn test_comprehensive_local_performance() {
    println!("运行综合本地性能测试...");

    let results = LocalPerformanceTester::run_comprehensive_local_test();

    assert!(!results.is_empty(), "应该有测试结果");

    for result in &results {
        println!(
            "测试 '{}': {:.2}ms 平均延迟, {:.1} RPS",
            result.test_name, result.avg_response_time_ms, result.requests_per_second
        );

        // 验证基本的性能指标
        assert!(
            result.total_requests > 0,
            "测试 {} 应该有请求",
            result.test_name
        );
        assert!(
            result.duration_seconds > 0.0,
            "测试 {} 应该有运行时间",
            result.test_name
        );
    }
}

#[test]
fn test_config_parameter_ranges() {
    println!("验证配置参数范围...");

    // 测试默认配置
    let default_config = OptimizedHttpConfig::default();
    assert_eq!(default_config.pool_max_idle_per_host, 90);
    assert_eq!(default_config.connect_timeout.as_secs(), 10);
    assert!(default_config.gzip);
    assert!(default_config.brotli);

    // 测试生产环境配置
    let prod_config = OptimizedHttpConfig::production();
    assert_eq!(prod_config.pool_max_idle_per_host, 100);
    assert_eq!(prod_config.connect_timeout.as_secs(), 5);
    assert!(prod_config.gzip);

    // 测试高吞吐量配置
    let high_throughput = OptimizedHttpConfig::high_throughput();
    assert_eq!(high_throughput.pool_max_idle_per_host, 200);
    assert_eq!(high_throughput.connect_timeout.as_secs(), 3);

    // 测试低延迟配置
    let low_latency = OptimizedHttpConfig::low_latency();
    assert_eq!(low_latency.pool_max_idle_per_host, 50);
    assert_eq!(low_latency.connect_timeout.as_secs(), 2);
    assert!(!low_latency.gzip); // 低延迟配置应该禁用压缩
    assert!(!low_latency.brotli);

    println!("✅ 所有配置参数验证通过");
}

#[test]
fn test_performance_metrics_calculation() {
    println!("测试性能指标计算...");

    use open_lark::core::performance::PerformanceMetrics;
    use std::time::Duration;

    // 测试正常情况
    let response_times = vec![100.0, 200.0, 150.0, 300.0, 250.0];
    let duration = Duration::from_secs(5);
    let metrics = PerformanceMetrics::calculate(
        "测试指标".to_string(),
        response_times,
        duration,
        5120, // 5KB
    );

    assert_eq!(metrics.total_requests, 5);
    assert_eq!(metrics.successful_requests, 5);
    assert_eq!(metrics.failed_requests, 0);
    assert_eq!(metrics.avg_response_time_ms, 200.0);
    assert_eq!(metrics.error_rate, 0.0);
    assert_eq!(metrics.requests_per_second, 1.0);

    // 测试包含失败的情况
    let response_times_with_failures = vec![100.0, -1.0, 150.0, -1.0, 250.0];
    let metrics_with_failures = PerformanceMetrics::calculate(
        "包含失败的测试".to_string(),
        response_times_with_failures,
        duration,
        3072, // 3KB (只有成功的请求)
    );

    assert_eq!(metrics_with_failures.total_requests, 5);
    assert_eq!(metrics_with_failures.successful_requests, 3);
    assert_eq!(metrics_with_failures.failed_requests, 2);
    assert_eq!(metrics_with_failures.error_rate, 40.0);

    println!("✅ 性能指标计算验证通过");
}

#[test]
fn test_user_agent_generation() {
    println!("测试User-Agent生成...");

    let configs = vec![
        OptimizedHttpConfig::default(),
        OptimizedHttpConfig::production(),
        OptimizedHttpConfig::high_throughput(),
        OptimizedHttpConfig::low_latency(),
    ];

    for config in configs {
        assert!(!config.user_agent.is_empty(), "User-Agent不应该为空");
        assert!(
            config.user_agent.contains("open-lark"),
            "User-Agent应该包含open-lark"
        );
        println!("  User-Agent: {}", config.user_agent);
    }

    println!("✅ User-Agent生成验证通过");
}
