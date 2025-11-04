//! ServiceRegistry 高级兼容性分析演示
//!
//! 展示智能兼容性分析、风险评估和迁移建议功能

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::service_registry::{
    AdvancedCompatibilityAnalyzer, CompatibilityAnalysisReport, CompatibilityConfig, MigrationHelper,
    ServiceRegistry, SharedConfig,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ServiceRegistry 高级兼容性分析演示");
    println!("======================================");

    // 1. 创建测试环境
    println!("📋 1. 创建测试环境");
    let config = ConfigBuilder::default()
        .app_id("advanced_demo_app")
        .app_secret("advanced_demo_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let shared_config = SharedConfig::new(config.clone());

    // 注册测试服务
    let _ = MigrationHelper::register_services_with_shared_config(&registry, &shared_config);
    println!("   ✅ 注册了 {} 个服务", registry.service_count());
    println!();

    // 2. 创建高级兼容性分析器
    println!("🔍 2. 创建高级兼容性分析器");
    let compatibility_config = CompatibilityConfig {
        strict_mode: false,
        allow_auto_downgrade: true,
        max_check_depth: 15,
        check_timeout: Duration::from_secs(60),
    };

    let analyzer = AdvancedCompatibilityAnalyzer::new(registry.clone(), compatibility_config);
    println!("   ✅ 高级兼容性分析器创建完成");
    println!();

    // 3. 执行全面的兼容性分析
    println!("📊 3. 执行全面的兼容性分析");
    let services = vec![
        "authentication-service".to_string(),
        "im-service".to_string(),
        "contact-service".to_string(),
        "group-service".to_string(),
        "search-service".to_string(),
    ];

    let analysis_report = analyzer.analyze_compatibility(&services);
    analysis_report.print();
    println!();

    // 4. 智能迁移策略推荐
    println!("🎯 4. 智能迁移策略推荐");
    let recommended_strategy = analyzer.recommend_migration_strategy(&services, &config);

    println!("推荐策略:");
    match &recommended_strategy.strategy {
        open_lark::service_registry::MigrationStrategy::Immediate => {
            println!("  🚀 立即迁移");
        }
        open_lark::service_registry::MigrationStrategy::Gradual { batch_size, delay_between_batches } => {
            println!("  📈 渐进式迁移 (批次大小: {}, 延迟: {:?})", batch_size, delay_between_batches);
        }
        open_lark::service_registry::MigrationStrategy::Canary { canary_services } => {
            println!("  🐤 金丝雀发布 (金丝雀服务: {:?})", canary_services);
        }
        open_lark::service_registry::MigrationStrategy::BlueGreen { validate_before_switch } => {
            println!("  🔄 蓝绿部署 (切换前验证: {})", validate_before_switch);
        }
    }

    println!("推荐理由: {}", recommended_strategy.reason);
    println!("置信度: {:.1}%", recommended_strategy.confidence * 100.0);
    println!("预估时间: {:?}", recommended_strategy.estimated_duration);
    println!();

    // 5. 风险评估和处理建议
    println!("⚠️  5. 风险评估和处理建议");
    demonstrate_risk_assessment(&analysis_report)?;
    println!();

    // 6. 迁移计划生成
    println!("📋 6. 迁移计划生成");
    demonstrate_migration_planning(&analyzer, &services, &config).await?;
    println!();

    // 7. 实时兼容性监控
    println!("📡 7. 实时兼容性监控");
    demonstrate_compatibility_monitoring(&analyzer, &services).await?;
    println!();

    // 8. 最佳实践建议
    println!("💡 8. 最佳实践建议");
    demonstrate_best_practices(&analysis_report);
    println!();

    println!("✅ 高级兼容性分析演示完成");
    println!("🎉 关键特性:");
    println!("   - 智能兼容性分析");
    println!("   - 风险自动识别");
    println!("   - 策略智能推荐");
    println!("   - 实时监控能力");
    println!("   - 最佳实践指导");

    Ok(())
}

/// 演示风险评估功能
fn demonstrate_risk_assessment(report: &CompatibilityAnalysisReport) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_risks = 0;
    let mut critical_risks = 0;
    let mut high_risks = 0;

    // 统计风险
    for analysis in report.service_analysis.values() {
        total_risks += analysis.risks.len();
        for risk in &analysis.risks {
            match risk.severity {
                open_lark::service_registry::compatibility::IssueSeverity::Critical => critical_risks += 1,
                open_lark::service_registry::compatibility::IssueSeverity::Error => high_risks += 1,
                _ => {}
            }
        }
    }

    println!("   风险统计:");
    println!("     🔴 关键风险: {} 个", critical_risks);
    println!("     🟠 高风险: {} 个", high_risks);
    println!("     📊 总风险数: {} 个", total_risks);

    if critical_risks > 0 {
        println!();
        println!("   🚨 发现关键风险，建议:");
        println!("     1. 立即处理关键风险项");
        println!("     2. 考虑推迟迁移直至风险解决");
        println!("     3. 制定详细的回滚计划");
        println!("     4. 增加监控和告警");
    } else if high_risks > 0 {
        println!();
        println!("   ⚠️  发现高风险项，建议:");
        println!("     1. 优先处理高风险问题");
        println!("     2. 使用保守的迁移策略");
        println!("     3. 减小迁移批次规模");
        println!("     4. 增加验证环节");
    } else {
        println!("   ✅ 风险评估通过，可以安全迁移");
    }

    Ok(())
}

/// 演示迁移计划生成
async fn demonstrate_migration_planning(
    analyzer: &AdvancedCompatibilityAnalyzer,
    services: &[String],
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // 获取推荐策略
    let strategy = analyzer.recommend_migration_strategy(services, config);

    println!("   基于分析结果生成的迁移计划:");
    println!("   📅 时间安排:");

    match &strategy.strategy {
        open_lark::service_registry::MigrationStrategy::Gradual { batch_size, .. } => {
            for (i, batch) in services.chunks(*batch_size).enumerate() {
                println!("     阶段 {}: {:?} (第 {} 分钟)",
                    i + 1, batch, i * 30 + 5);
            }
        }
        open_lark::service_registry::MigrationStrategy::Canary { canary_services } => {
            println!("     阶段 1: 金丝雀服务 {:?} (第 10 分钟)", canary_services);
            println!("     阶段 2: 验证金丝雀 (第 20 分钟)");
            println!("     阶段 3: 全量迁移 (第 30 分钟)");
        }
        open_lark::service_registry::MigrationStrategy::BlueGreen { .. } => {
            println!("     阶段 1: 绿色环境部署 (第 15 分钟)");
            println!("     阶段 2: 绿色环境验证 (第 25 分钟)");
            println!("     阶段 3: 流量切换 (第 30 分钟)");
        }
        open_lark::service_registry::MigrationStrategy::Immediate => {
            println!("     立即执行: 所有服务 (第 5 分钟)");
        }
    }

    println!();
    println!("   🎯 关键检查点:");
    println!("     ✓ 迁移前备份验证");
    println!("     ✓ 每个阶段后健康检查");
    println!("     ✓ 关键指标监控");
    println!("     ✓ 回滚条件确认");

    Ok(())
}

/// 演示兼容性监控
async fn demonstrate_compatibility_monitoring(
    analyzer: &AdvancedCompatibilityAnalyzer,
    services: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   模拟实时兼容性监控...");

    // 模拟监控过程
    for i in 1..=5 {
        tokio::time::sleep(Duration::from_millis(200)).await;

        // 模拟不同的监控结果
        let report = if i <= 3 {
            analyzer.analyze_compatibility(services)
        } else {
            // 模拟出现问题
            let mut report = analyzer.analyze_compatibility(services);
            if let Some(analysis) = report.service_analysis.values_mut().next() {
                analysis.risks.push(open_lark::service_registry::ServiceRisk {
                    risk_type: open_lark::service_registry::ServiceRiskType::ConfigurationIssue,
                    severity: open_lark::service_registry::compatibility::IssueSeverity::Warning,
                    description: "检测到配置漂移".to_string(),
                    impact: "可能影响服务间通信".to_string(),
                    mitigation: "同步配置文件".to_string(),
                });
            }
            report
        };

        let compatible_count = report.service_analysis.values()
            .filter(|s| matches!(s.compatibility_level, open_lark::service_registry::compatibility::CompatibilityLevel::Full))
            .count();

        println!("     监控点 {}: {}/{} 服务兼容 (状态: {})",
            i, compatible_count, services.len(),
            if compatible_count == services.len() { "✅ 正常" } else { "⚠️ 异常" });
    }

    println!("   📊 监控总结:");
    println!("     - 监控时长: 1 秒 (模拟)");
    println!("     - 检查频率: 200ms");
    println!("     - 异常检测: ✅ 支持");
    println!("     - 实时告警: ✅ 支持");

    Ok(())
}

/// 演示最佳实践
fn demonstrate_best_practices(report: &CompatibilityAnalysisReport) {
    println!("   基于分析结果的最佳实践建议:");

    // 基于服务数量提供建议
    if report.total_services <= 5 {
        println!("     🎯 小规模部署建议:");
        println!("       - 使用立即迁移策略");
        println!("       - 确保有完整备份");
        println!("       - 执行前进行完整测试");
    } else if report.total_services <= 20 {
        println!("     🎯 中等规模部署建议:");
        println!("       - 使用渐进式迁移策略");
        println!("       - 分批次处理");
        println!("       - 每批次后验证");
    } else {
        println!("     🎯 大规模部署建议:");
        println!("       - 使用金丝雀或蓝绿部署");
        println!("       - 延长验证时间");
        println!("       - 增强监控和告警");
    }

    // 基于依赖复杂度提供建议
    if !report.cross_service_dependencies.is_empty() {
        println!();
        println!("     🔗 依赖管理建议:");
        println!("       - 按依赖顺序迁移");
        println!("       - 考虑服务解耦");
        println!("       - 实施熔断机制");
    }

    // 基于问题数量提供建议
    let total_issues = report.service_analysis.values()
        .map(|s| s.issues.len())
        .sum::<usize>();

    if total_issues > 0 {
        println!();
        println!("     🔧 问题解决建议:");
        println!("       - 优先处理关键问题");
        println!("       - 建立问题跟踪机制");
        println!("       - 制定解决时间表");
    }

    println!();
    println!("     📋 通用最佳实践:");
    println!("       - 制定详细的迁移计划");
    println!("       - 准备回滚方案");
    println!("       - 建立沟通机制");
    println!("       - 进行充分测试");
    println!("       - 监控关键指标");
}