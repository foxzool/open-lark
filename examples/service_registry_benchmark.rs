//! ServiceRegistry性能基准测试示例
//!
//! 展示ServiceRegistry在各种场景下的性能表现

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::service_registry::benchmark::BenchmarkSuite;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ServiceRegistry 性能基准测试");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("测试时间: {}", timestamp);
    println!();

    // 创建配置
    let config = ConfigBuilder::default()
        .app_id("benchmark_app")
        .app_secret("benchmark_secret")
        .build();

    // 创建基准测试套件
    let suite = BenchmarkSuite::with_config(config);

    // 设置核心服务
    println!("📋 设置核心服务...");
    suite
        .setup_core_services()
        .expect("Failed to setup core services");
    println!("✅ 核心服务设置完成");
    println!();

    // 运行完整基准测试套件
    let results = suite.run_full_benchmark_suite().await;

    // 性能总结
    println!("📊 性能分析报告");
    println!("{}", "-".repeat(50));

    for (i, result) in results.iter().enumerate() {
        let status = if result.avg_duration_nanos < 1000 {
            "✅ 优秀"
        } else if result.avg_duration_nanos < 5000 {
            "🟡 良好"
        } else {
            "⚠️  需要优化"
        };

        println!("{}. {} [{}]", i + 1, result.test_name, status);
        println!("   📈 性能: {:.0} ops/sec", result.ops_per_second);
        println!(
            "   ⏱️  平均耗时: {:.2} μs",
            result.avg_duration_nanos as f64 / 1000.0
        );

        if let Some(memory) = result.memory_usage_bytes {
            println!("   💾 内存使用: {:.2} KB", memory as f64 / 1024.0);
        }
        println!();
    }

    // 性能建议
    println!("💡 性能优化建议");
    println!("{}", "-".repeat(50));

    for result in &results {
        if result.avg_duration_nanos > 10_000 {
            // 10微秒
            println!(
                "⚠️  {}: 平均耗时 {:.2} μs 超过建议值，建议检查实现",
                result.test_name,
                result.avg_duration_nanos as f64 / 1000.0
            );
        }
    }

    if results.iter().all(|r| r.avg_duration_nanos < 10_000) {
        println!("✅ 所有性能指标都在建议范围内，ServiceRegistry性能表现良好！");
    }

    println!();
    println!("🎯 基准测试完成！");
    println!("📝 如需详细分析，请查看上面的性能数据");

    Ok(())
}
