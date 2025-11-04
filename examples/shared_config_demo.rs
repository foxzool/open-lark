//! ServiceRegistry共享配置演示
//!
//! 展示如何使用SharedConfig来优化配置管理和内存使用

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::service_registry::{SharedConfig, SharedConfigFactory, ConfigUsageStats, ServiceRegistry, MigrationHelper};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ServiceRegistry 共享配置演示");
    println!("==============================");

    // 1. 创建基础配置
    println!("📋 1. 创建基础配置");
    let config = ConfigBuilder::default()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    println!("✅ 基础配置创建完成");
    println!("   App ID: {}", config.app_id);
    println!("   Base URL: {}", config.base_url);
    println!();

    // 2. 演示传统方式 vs 共享方式
    println!("📊 2. 配置使用方式对比");

    // 传统方式：每个服务独立持有配置
    let traditional_configs: Vec<Config> = (0..5).map(|_| config.clone()).collect();
    let traditional_memory = traditional_configs.len() * std::mem::size_of::<Config>();

    // 共享方式：所有服务共享同一个配置
    let shared_config = SharedConfig::new(config.clone());
    let shared_configs: Vec<SharedConfig> = (0..5).map(|_| shared_config.clone_shared()).collect();
    let shared_memory = std::mem::size_of::<Config>() + (shared_configs.len() * std::mem::size_of::<SharedConfig>());

    let memory_saved = traditional_memory.saturating_sub(shared_memory);
    let savings_percentage = if traditional_memory > 0 {
        (memory_saved as f64 / traditional_memory as f64) * 100.0
    } else {
        0.0
    };

    println!("   传统方式: {} bytes", traditional_memory);
    println!("   共享方式: {} bytes", shared_memory);
    println!("   内存节省: {} bytes ({:.1}%)", memory_saved, savings_percentage);
    println!("   引用计数: {}", shared_config.ref_count());
    println!();

    // 3. 演示工厂方法
    println!("🏭 3. 共享配置工厂方法");

    // 使用工厂创建共享配置
    let factory_config = SharedConfigFactory::create_shared(config.clone());
    println!("   工厂创建: {} 引用", factory_config.ref_count());

    // 批量创建服务配置
    let service_names = vec!["auth-service", "im-service", "contact-service"];
    let (main_config, service_configs) = SharedConfigFactory::create_batch(config.clone(), &service_names);

    println!("   批量创建: {} 个服务配置", service_configs.len());
    println!("   主配置引用: {}", main_config.ref_count());
    println!();

    // 4. 演示在ServiceRegistry中的使用
    println!("📋 4. ServiceRegistry集成");

    let registry = ServiceRegistry::new();

    // 使用传统方式注册服务
    println!("   传统方式注册服务...");
    let result_traditional = MigrationHelper::register_services(&registry, &config.clone());
    if result_traditional.is_ok() {
        println!("   ✅ 传统方式注册成功");
    }

    // 清理注册表
    cleanup_registry(&registry);

    // 使用共享配置方式注册服务
    println!("   共享配置方式注册服务...");
    let result_shared = MigrationHelper::register_services_with_shared_config(&registry, &shared_config);
    if result_shared.is_ok() {
        println!("   ✅ 共享配置方式注册成功");
        println!("   共享配置引用计数: {}", shared_config.ref_count());
    }
    println!();

    // 5. 演示配置使用统计
    println!("📊 5. 配置使用统计");

    let stats = ConfigUsageStats::new(&shared_config, 5);
    stats.print();

    // 6. 演示大规模场景
    println!("🚀 6. 大规模场景演示");

    let large_shared_config = SharedConfig::new(ConfigBuilder::default()
        .app_id("large_scale_app")
        .app_secret("large_scale_secret")
        .build());

    // 模拟100个服务
    let large_service_count = 100;
    let large_configs: Vec<SharedConfig> = (0..large_service_count)
        .map(|_| large_shared_config.clone_shared())
        .collect();

    let large_stats = ConfigUsageStats::new(&large_shared_config, large_service_count);
    println!("   大规模场景 - {} 个服务", large_service_count);
    large_stats.print();

    // 7. 并发访问演示
    println!("⚡ 7. 并发访问演示");

    let concurrent_config = std::sync::Arc::new(SharedConfig::new(ConfigBuilder::default()
        .app_id("concurrent_app")
        .app_secret("concurrent_secret")
        .build()));

    // 使用标准库的并发测试
    use std::thread;

    let mut handles = vec![];

    for i in 0..10 {
        let config_clone = std::sync::Arc::clone(&concurrent_config);
        let handle = thread::spawn(move || {
            // 模拟并发访问配置
            let app_id = config_clone.config().app_id.clone();
            let ref_count = config_clone.ref_count();

            // 模拟一些处理时间
            thread::sleep(std::time::Duration::from_millis(1));

            println!("   并发任务 {}: AppID={}, 引用计数={}", i, app_id, ref_count);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   并发任务完成: 10 个");
    println!("   最终引用计数: {}", concurrent_config.ref_count());
    println!("   最终引用计数: {}", concurrent_config.ref_count());
    println!();

    // 8. 总结
    println!("📋 8. 总结");
    println!("==============================");
    println!("✅ 共享配置功能演示完成");
    println!("🔧 主要优势:");
    println!("   - 内存使用优化（特别是在大量服务场景）");
    println!("   - 配置一致性保证");
    println!("   - 线程安全的并发访问");
    println!("   - 简化的配置管理");
    println!("   - 为未来服务优化奠定基础");
    println!();

    println!("💡 使用建议:");
    println!("   - 在服务数量较多时优先考虑共享配置");
    println!("   - 使用工厂方法简化配置创建");
    println!("   - 定期监控配置使用统计");
    println!("   - 在异步环境中注意线程安全");

    Ok(())
}

fn cleanup_registry(registry: &ServiceRegistry) {
    // 清理所有注册的服务
    let services = registry.discover_services();
    for service_name in services {
        let _ = registry.unregister(service_name);
    }
}