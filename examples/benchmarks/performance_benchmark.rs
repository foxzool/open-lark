//! HTTP客户端性能基准测试示例
//!
//! 此示例展示如何：
//! 1. 使用不同的HTTP客户端配置
//! 2. 运行性能基准测试
//! 3. 比较配置性能差异
//! 4. 选择最佳配置
//!
//! 运行方式：
//! ```bash
//! cargo run --example performance_benchmark --features=benchmarks
//! ```

use open_lark::core::{
    config::Config,
    performance::{BenchmarkConfig, ClientComparison, HttpBenchmark, OptimizedHttpConfig},
};
use std::collections::HashMap;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志记录
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("🚀 开始HTTP客户端性能基准测试");

    // 测试配置
    let benchmark_config = BenchmarkConfig {
        concurrent_connections: 20,
        requests_per_connection: 50,
        warmup_requests: 10,
        target_url: "https://httpbin.org/json".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("User-Agent".to_string(), "open-lark-benchmark".to_string());
            headers
        },
    };

    // 配置1: 默认配置
    let default_config = OptimizedHttpConfig::default();

    // 配置2: 生产环境配置
    let production_config = OptimizedHttpConfig::production();

    // 配置3: 高吞吐量配置
    let high_throughput_config = OptimizedHttpConfig::high_throughput();

    // 配置4: 低延迟配置
    let low_latency_config = OptimizedHttpConfig::low_latency();

    // 配置5: 自定义极速配置
    let ultra_fast_config = OptimizedHttpConfig {
        pool_max_idle_per_host: 300,
        pool_idle_timeout: std::time::Duration::from_secs(300),
        connect_timeout: std::time::Duration::from_millis(1000),
        request_timeout: std::time::Duration::from_secs(5),
        http2_prior_knowledge: true,
        http2_adaptive_window: true,
        gzip: false,
        brotli: false,
        user_agent: "open-lark-ultra-fast".to_string(),
    };

    // 要测试的配置列表
    let configs = vec![
        ("默认配置", default_config),
        ("生产环境", production_config),
        ("高吞吐量", high_throughput_config),
        ("低延迟", low_latency_config),
        ("极速自定义", ultra_fast_config),
    ];

    info!("📊 开始配置性能比较测试...");

    // 运行配置比较
    match ClientComparison::compare_configurations(configs, benchmark_config.clone()).await {
        Ok(results) => {
            info!("✅ 所有配置测试完成");

            // 输出推荐配置
            print_recommendations(&results);
        }
        Err(e) => {
            eprintln!("❌ 配置比较测试失败: {}", e);
        }
    }

    // 演示如何在实际应用中使用优化配置
    info!("🔧 演示在Config中集成优化配置...");
    demonstrate_config_integration().await?;

    info!("🎉 性能基准测试完成！");
    Ok(())
}

/// 打印配置推荐
fn print_recommendations(results: &[(String, open_lark::core::performance::PerformanceMetrics)]) {
    info!("╔══════════════════════════════════════════════════════════════════════════╗");
    info!("║                            📊 配置推荐                                  ║");
    info!("╚══════════════════════════════════════════════════════════════════════════╝");

    if let Some((best_throughput_name, best_throughput)) = results.iter().max_by(|a, b| {
        a.1.requests_per_second
            .partial_cmp(&b.1.requests_per_second)
            .unwrap()
    }) {
        info!(
            "🏆 最佳吞吐量: {} ({:.1} RPS)",
            best_throughput_name, best_throughput.requests_per_second
        );
        info!("   推荐场景: 批量数据处理、后台同步任务");
    }

    if let Some((best_latency_name, best_latency)) = results.iter().min_by(|a, b| {
        a.1.avg_response_time_ms
            .partial_cmp(&b.1.avg_response_time_ms)
            .unwrap()
    }) {
        info!(
            "🚀 最低延迟: {} ({:.2}ms 平均延迟)",
            best_latency_name, best_latency.avg_response_time_ms
        );
        info!("   推荐场景: 实时API调用、用户交互功能");
    }

    if let Some((most_reliable_name, most_reliable)) = results
        .iter()
        .min_by(|a, b| a.1.error_rate.partial_cmp(&b.1.error_rate).unwrap())
    {
        info!(
            "🛡️  最高可靠性: {} ({:.2}% 错误率)",
            most_reliable_name, most_reliable.error_rate
        );
        info!("   推荐场景: 关键业务流程、金融交易");
    }

    // 提供具体的使用建议
    info!("");
    info!("📋 使用建议:");
    info!("   1. 开发/测试环境: 使用默认配置");
    info!("   2. 生产环境 (通用): 使用生产环境配置");
    info!("   3. 高并发场景: 使用高吞吐量配置");
    info!("   4. 实时应用: 使用低延迟配置");
    info!("   5. 特殊需求: 基于基准测试结果定制配置");
}

/// 演示在Config中集成优化配置
async fn demonstrate_config_integration() -> Result<(), Box<dyn std::error::Error>> {
    info!("创建使用生产环境优化配置的SDK配置...");

    // 方式1: 使用预设配置
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .production_http_client()?
        .build();

    info!("✅ 生产环境配置创建成功");

    // 方式2: 使用自定义配置
    let custom_http_config = OptimizedHttpConfig {
        pool_max_idle_per_host: 150,
        pool_idle_timeout: std::time::Duration::from_secs(120),
        connect_timeout: std::time::Duration::from_secs(3),
        request_timeout: std::time::Duration::from_secs(20),
        http2_prior_knowledge: true,
        http2_adaptive_window: true,
        gzip: true,
        brotli: true,
        user_agent: "my-custom-app/1.0".to_string(),
    };

    let _custom_config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .optimized_http_client(custom_http_config)?
        .build();

    info!("✅ 自定义优化配置创建成功");

    // 输出配置信息
    info!("配置引用计数: {}", config.reference_count());
    info!("HTTP客户端池配置: 已优化");

    Ok(())
}

/// 运行单个配置的详细基准测试
#[allow(dead_code)]
async fn run_detailed_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    info!("🔍 运行详细基准测试...");

    let config = OptimizedHttpConfig::production();
    let client = config.build_client()?;

    let benchmark_config = BenchmarkConfig {
        concurrent_connections: 50,
        requests_per_connection: 200,
        warmup_requests: 50,
        target_url: "https://httpbin.org/json".to_string(),
        headers: HashMap::new(),
    };

    let benchmark = HttpBenchmark::new(client, benchmark_config);
    let metrics = benchmark
        .run_benchmark()
        .await
        .map_err(|e| format!("Benchmark failed: {}", e))?;

    info!("📈 详细基准测试结果:");
    metrics.print_report();

    Ok(())
}
