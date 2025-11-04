//! ServiceRegistry 兼容性和迁移演示
//!
//! 展示兼容性检查、版本管理和高级迁移功能

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::service_registry::{
    AdvancedMigrationHelper, CompatibilityChecker, CompatibilityConfig, CompatibilityHandler,
    MigrationStrategy, ServiceRegistry, ServiceVersion, SharedConfig,
};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ServiceRegistry 兼容性和迁移演示");
    println!("=====================================");

    // 1. 创建测试配置
    println!("📋 1. 创建测试配置");
    let source_config = ConfigBuilder::default()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let target_config = ConfigBuilder::default()
        .app_id("demo_app_id_v2")
        .app_secret("demo_app_secret_v2")
        .base_url("https://open.feishu.cn")
        .build();

    println!("   源配置: AppID={}", source_config.app_id);
    println!("   目标配置: AppID={}", target_config.app_id);
    println!();

    // 2. 版本兼容性演示
    println!("📊 2. 版本兼容性演示");
    demonstrate_version_compatibility()?;

    // 3. 兼容性检查演示
    println!("🔍 3. 兼容性检查演示");
    demonstrate_compatibility_checking(&source_config, &target_config).await?;

    // 4. 迁移策略演示
    println!("🔄 4. 迁移策略演示");
    demonstrate_migration_strategies(&source_config, &target_config).await?;

    // 5. 实际迁移演示
    println!("⚡ 5. 实际迁移演示");
    demonstrate_actual_migration(&source_config, &target_config).await?;

    // 6. 错误处理和回滚演示
    println!("🛡️  6. 错误处理和回滚演示");
    demonstrate_error_handling_and_rollback().await?;

    // 7. 大规模迁移演示
    println!("🚀 7. 大规模迁移演示");
    demonstrate_large_scale_migration(&source_config, &target_config).await?;

    println!();
    println!("✅ 兼容性和迁移演示完成");
    println!("💡 关键特性:");
    println!("   - 版本兼容性检查");
    println!("   - 多种迁移策略支持");
    println!("   - 实时迁移监控");
    println!("   - 自动回滚机制");
    println!("   - 风险识别和建议");
    println!("   - 大规模部署支持");

    Ok(())
}

fn demonstrate_version_compatibility() -> Result<(), Box<dyn std::error::Error>> {
    // 创建不同版本
    let v1_0_0 = ServiceVersion::new(1, 0, 0);
    let v1_1_0 = ServiceVersion::new(1, 1, 0);
    let v1_2_3 = ServiceVersion::new(1, 2, 3);
    let v2_0_0 = ServiceVersion::new(2, 0, 0);
    let v1_0_0_beta = ServiceVersion::pre_release(1, 0, 0, "beta".to_string());

    println!("   版本解析:");
    println!("     1.0.0 -> {:?}", v1_0_0);
    println!("     1.2.3-beta -> {:?}", v1_0_0_beta);
    println!();

    println!("   兼容性检查 (非严格模式):");
    println!("     1.2.3 兼容 1.1.0: {}", v1_2_3.is_compatible_with(&v1_1_0, false));
    println!("     1.1.0 兼容 1.2.3: {}", v1_1_0.is_compatible_with(&v1_2_3, false));
    println!("     1.0.0 兼容 2.0.0: {}", v1_0_0.is_compatible_with(&v2_0_0, false));
    println!();

    println!("   兼容性检查 (严格模式):");
    println!("     1.2.3 兼容 1.2.3: {}", v1_2_3.is_compatible_with(&v1_2_3, true));
    println!("     1.2.3 兼容 1.1.0: {}", v1_2_3.is_compatible_with(&v1_1_0, true));
    println!();

    Ok(())
}

async fn demonstrate_compatibility_checking(
    source_config: &Config,
    target_config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let compatibility_config = CompatibilityConfig::default();
    let checker = CompatibilityChecker::new(compatibility_config.clone());
    let handler = CompatibilityHandler::new(registry.clone(), compatibility_config);

    // 注册一些测试服务
    let shared_config = SharedConfig::new(source_config.clone());
    let _ = open_lark::service_registry::MigrationHelper::register_services_with_shared_config(
        &registry,
        &shared_config,
    );

    // 检查服务兼容性
    let service_version = ServiceVersion::new(1, 0, 0);
    let result = checker.check_service_compatibility("authentication-service", &service_version, &registry)?;

    println!("   服务兼容性检查:");
    println!("     服务: authentication-service");
    println!("     版本: {}", service_version.to_string());
    println!("     兼容性: {:?}", result.compatibility_level);
    println!("     问题数量: {}", result.issues.len());

    if !result.issues.is_empty() {
        println!("     发现的问题:");
        for issue in &result.issues {
            println!("       - {:?}", issue.issue_type);
        }
    }
    println!();

    // 检查配置兼容性
    let config_result = checker.check_config_compatibility(target_config, &registry)?;
    println!("   配置兼容性检查:");
    println!("     兼容性: {:?}", config_result.compatibility_level);
    println!("     问题数量: {}", config_result.issues.len());

    if !config_result.issues.is_empty() {
        println!("     配置问题:");
        for issue in &config_result.issues {
            println!("       - {}", issue.description);
        }
    }
    println!();

    // 生成兼容性报告
    let report = handler.generate_compatibility_report();
    report.print_summary();

    Ok(())
}

async fn demonstrate_migration_strategies(
    source_config: &Config,
    target_config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let compatibility_config = CompatibilityConfig::default();
    let migration_helper = AdvancedMigrationHelper::new(registry.clone(), compatibility_config);

    let services = vec![
        "authentication-service".to_string(),
        "im-service".to_string(),
        "contact-service".to_string(),
    ];

    // 渐进式迁移策略
    println!("   渐进式迁移策略:");
    let gradual_strategy = MigrationStrategy::Gradual {
        batch_size: 2,
        delay_between_batches: Duration::from_millis(500),
    };
    let plan1 = migration_helper.generate_migration_plan(
        &services,
        gradual_strategy,
        source_config,
        target_config,
    );
    plan1.print();

    // 金丝雀发布策略
    println!("   金丝雀发布策略:");
    let canary_strategy = MigrationStrategy::Canary {
        canary_services: vec!["authentication-service".to_string()],
    };
    let plan2 = migration_helper.generate_migration_plan(
        &services,
        canary_strategy,
        source_config,
        target_config,
    );
    plan2.print();

    // 立即迁移策略
    println!("   立即迁移策略:");
    let immediate_strategy = MigrationStrategy::Immediate;
    let plan3 = migration_helper.generate_migration_plan(
        &services,
        immediate_strategy,
        source_config,
        target_config,
    );
    plan3.print();

    Ok(())
}

async fn demonstrate_actual_migration(
    source_config: &Config,
    target_config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let compatibility_config = CompatibilityConfig::default();
    let migration_helper = AdvancedMigrationHelper::new(registry.clone(), compatibility_config);

    // 先注册一些服务
    let shared_source_config = SharedConfig::new(source_config.clone());
    let _ = open_lark::service_registry::MigrationHelper::register_services_with_shared_config(
        &registry,
        &shared_source_config,
    );

    println!("   迁移前服务数量: {}", registry.service_count());

    // 执行渐进式迁移
    let services = vec!["authentication-service".to_string(), "im-service".to_string()];
    let task_id = "demo-migration-1".to_string();
    let strategy = MigrationStrategy::Gradual {
        batch_size: 1,
        delay_between_batches: Duration::from_millis(100),
    };

    println!("   开始迁移任务...");
    let migration_id = migration_helper
        .start_migration(
            task_id.clone(),
            strategy,
            services.clone(),
            source_config.clone(),
            target_config.clone(),
        )
        .await?;

    println!("   迁移任务ID: {}", migration_id);

    // 监控迁移进度
    let mut last_progress = 0.0;
    for _ in 0..20 {
        tokio::time::sleep(Duration::from_millis(200)).await;

        if let Some(task) = migration_helper.get_migration_status(&migration_id).await {
            if let open_lark::service_registry::MigrationStatus::InProgress { progress } = task.status {
                if (progress - last_progress).abs() > 0.1 {
                    println!("   迁移进度: {:.1}%", progress);
                    last_progress = progress;
                }
            } else if matches!(task.status, open_lark::service_registry::MigrationStatus::Completed) {
                println!("   ✅ 迁移完成！");
                break;
            } else if let open_lark::service_registry::MigrationStatus::Failed { error } = &task.status {
                println!("   ❌ 迁移失败: {}", error);
                break;
            }
        }
    }

    println!("   迁移后服务数量: {}", registry.service_count());
    println!();

    Ok(())
}

async fn demonstrate_error_handling_and_rollback() -> Result<(), Box<dyn std::error::Error>> {
    println!("   模拟迁移失败场景...");

    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let compatibility_config = CompatibilityConfig {
        strict_mode: true,
        allow_auto_downgrade: false,
        ..Default::default()
    };
    let migration_helper = AdvancedMigrationHelper::new(registry.clone(), compatibility_config);

    // 创建一个不兼容的目标配置
    let incompatible_config = ConfigBuilder::default()
        .app_id("") // 空 App ID，会导致兼容性检查失败
        .app_secret("")
        .build();

    let services = vec!["test-service".to_string()];
    let task_id = "demo-migration-fail".to_string();
    let strategy = MigrationStrategy::Immediate;

    println!("   尝试使用不兼容配置进行迁移...");
    let result = migration_helper
        .start_migration(
            task_id.clone(),
            strategy,
            services,
            ConfigBuilder::default().build(),
            incompatible_config,
        )
        .await;

    match result {
        Ok(_) => {
            println!("   ⚠️  预期失败但成功了");
        }
        Err(e) => {
            println!("   ✅ 正确捕获到错误: {}", e);
        }
    }

    println!();

    Ok(())
}

async fn demonstrate_large_scale_migration(
    source_config: &Config,
    target_config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   模拟大规模迁移场景...");

    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let compatibility_config = CompatibilityConfig::default();
    let migration_helper = AdvancedMigrationHelper::new(registry.clone(), compatibility_config);

    // 生成大量模拟服务
    let services: Vec<String> = (0..20)
        .map(|i| format!("service-{:03}", i))
        .collect();

    println!("   服务数量: {}", services.len());

    let task_id = "demo-large-migration".to_string();
    let strategy = MigrationStrategy::Gradual {
        batch_size: 5,
        delay_between_batches: Duration::from_millis(100),
    };

    let start_time = std::time::Instant::now();
    let migration_id = migration_helper
        .start_migration(
            task_id.clone(),
            strategy,
            services.clone(),
            source_config.clone(),
            target_config.clone(),
        )
        .await?;

    println!("   大规模迁移任务开始: {}", migration_id);

    // 监控大规模迁移
    for _ in 0..50 {
        tokio::time::sleep(Duration::from_millis(200)).await;

        if let Some(task) = migration_helper.get_migration_status(&migration_id).await {
            if let open_lark::service_registry::MigrationStatus::InProgress { progress } = task.status {
                if progress > 99.0 {
                    println!("   🎉 大规模迁移即将完成: {:.1}%", progress);
                    break;
                }
            } else if matches!(task.status, open_lark::service_registry::MigrationStatus::Completed) {
                println!("   ✅ 大规模迁移完成！");
                break;
            }
        }
    }

    let duration = start_time.elapsed();
    println!("   总耗时: {:?}", duration);

    // 清理完成的迁移任务
    let cleaned_count = migration_helper.cleanup_completed_migrations().await;
    println!("   清理了 {} 个已完成的迁移任务", cleaned_count);
    println!();

    Ok(())
}