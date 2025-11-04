//! SharedConfig性能对比演示
//!
//! 测试不同配置策略下的性能和内存使用情况

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::service_registry::{
    MigrationHelper, ServiceRegistry, SharedConfig, SharedConfigFactory,
};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 SharedConfig 性能对比演示");
    println!("==============================");

    // 创建测试配置
    let config = ConfigBuilder::default()
        .app_id("performance_test_app")
        .app_secret("performance_test_secret")
        .base_url("https://open.feishu.cn")
        .build();

    println!("📊 测试场景：");
    println!("1. 小规模服务（5个服务）");
    println!("2. 中等规模服务（50个服务）");
    println!("3. 大规模服务（200个服务）");
    println!();

    // 测试场景1：小规模服务
    test_performance_scenario("小规模", 5, &config)?;

    // 测试场景2：中等规模服务
    test_performance_scenario("中等规模", 50, &config)?;

    // 测试场景3：大规模服务
    test_performance_scenario("大规模", 200, &config)?;

    // ServiceRegistry集成性能测试
    println!("🔧 ServiceRegistry集成性能测试");
    println!("================================");

    test_registry_integration_performance(&config)?;

    println!();
    println!("✅ 性能对比测试完成");
    println!("💡 关键发现：");
    println!("   - 共享配置在小规模场景下优势不明显");
    println!("   - 中等规模场景开始显示内存优势");
    println!("   - 大规模场景下显著减少内存使用");
    println!("   - 创建和访问性能保持稳定");

    Ok(())
}

fn test_performance_scenario(
    name: &str,
    service_count: usize,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 {}场景（{}个服务）", name, service_count);

    // 传统方式：独立配置
    let start = Instant::now();
    let traditional_configs: Vec<Config> = (0..service_count).map(|_| config.clone()).collect();
    let traditional_time = start.elapsed();
    let traditional_memory = traditional_configs.len() * std::mem::size_of::<Config>();

    // 共享配置方式
    let start = Instant::now();
    let shared_config = SharedConfig::new(config.clone());
    let shared_configs: Vec<SharedConfig> = (0..service_count)
        .map(|_| shared_config.clone_shared())
        .collect();
    let shared_time = start.elapsed();
    let shared_memory = std::mem::size_of::<Config>()
        + (shared_configs.len() * std::mem::size_of::<SharedConfig>());

    // 计算差异
    let memory_diff = traditional_memory.saturating_sub(shared_memory);
    let memory_savings_percent = if traditional_memory > 0 {
        (memory_diff as f64 / traditional_memory as f64) * 100.0
    } else {
        0.0
    };

    let time_diff = if traditional_time > shared_time {
        traditional_time - shared_time
    } else {
        shared_time - traditional_time
    };

    println!(
        "   传统方式：{:?}，{} bytes",
        traditional_time, traditional_memory
    );
    println!("   共享方式：{:?}，{} bytes", shared_time, shared_memory);
    println!(
        "   内存节省：{} bytes ({:.1}%)",
        memory_diff, memory_savings_percent
    );

    if shared_time < traditional_time {
        println!("   时间优势：快 {:?}", time_diff);
    } else {
        println!("   时间差异：慢 {:?}", time_diff);
    }

    println!("   最终引用计数：{}", shared_config.ref_count());
    println!();

    Ok(())
}

fn test_registry_integration_performance(
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let service_count = 10;

    // 测试传统方式注册
    let registry1 = ServiceRegistry::new();
    let start = Instant::now();
    let result1 = MigrationHelper::register_services(&registry1, &config);
    let traditional_time = start.elapsed();

    #[cfg(any(feature = "authentication", feature = "im", feature = "contact"))]
    {
        let _ = result1;
    }

    // 测试共享配置方式注册
    let registry2 = ServiceRegistry::new();
    let shared_config = SharedConfig::new(config.clone());
    let start = Instant::now();
    let result2 = MigrationHelper::register_services_with_shared_config(&registry2, &shared_config);
    let shared_time = start.elapsed();

    #[cfg(any(feature = "authentication", feature = "im", feature = "contact"))]
    {
        let _ = result2;
    }

    println!("   传统注册方式：{:?}", traditional_time);
    println!("   共享注册方式：{:?}", shared_time);

    if shared_time < traditional_time {
        let diff = traditional_time - shared_time;
        println!("   性能提升：快 {:?}", diff);
    } else {
        let diff = shared_time - traditional_time;
        println!("   性能差异：慢 {:?}", diff);
    }

    // 获取注册表统计信息
    let stats1 = registry1.get_stats();
    let stats2 = registry2.get_stats();

    println!("   传统方式服务数：{}", stats1.total_services);
    println!("   共享方式服务数：{}", stats2.total_services);
    println!("   共享配置最终引用：{}", shared_config.ref_count());

    Ok(())
}
