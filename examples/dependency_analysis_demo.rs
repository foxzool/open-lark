//! ServiceRegistry 深度依赖分析演示
//!
//! 展示服务依赖分析、循环依赖检测和迁移影响评估功能

use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::service_registry::{
    CircularDependencySeverity, CriticalPathType, DependencyAnalysisReport, DependencyAnalyzer,
    MigrationHelper, RecommendationPriority, RiskLevel, ServiceRegistry, SharedConfig,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ServiceRegistry 深度依赖分析演示");
    println!("==================================");

    // 1. 创建测试环境
    println!("📋 1. 创建测试环境");
    let config = ConfigBuilder::default()
        .app_id("dependency_demo_app")
        .app_secret("dependency_demo_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let registry = std::sync::Arc::new(ServiceRegistry::new());
    let shared_config = SharedConfig::new(config.clone());

    // 注册测试服务
    let _ = MigrationHelper::register_services_with_shared_config(&registry, &shared_config);
    println!("   ✅ 注册了 {} 个服务", registry.service_count());
    println!();

    // 2. 创建依赖分析器
    println!("🔍 2. 创建依赖分析器");
    let analyzer = DependencyAnalyzer::new(registry.clone());
    println!("   ✅ 依赖分析器创建完成");
    println!();

    // 3. 执行全面的依赖分析
    println!("📊 3. 执行全面的依赖分析");
    let dependency_report = analyzer.analyze_dependencies();
    dependency_report.print();
    println!();

    // 4. 循环依赖检测演示
    println!("🔄 4. 循环依赖检测演示");
    demonstrate_circular_dependency_detection(&dependency_report)?;
    println!();

    // 5. 关键路径分析
    println!("🎯 5. 关键路径分析");
    demonstrate_critical_path_analysis(&dependency_report)?;
    println!();

    // 6. 迁移影响评估
    println!("⚡ 6. 迁移影响评估");
    demonstrate_migration_impact_analysis(&analyzer).await?;
    println!();

    // 7. 依赖图可视化数据生成
    println!("🎨 7. 依赖图可视化数据生成");
    demonstrate_dependency_graph_generation(&analyzer)?;
    println!();

    // 8. 架构优化建议
    println!("💡 8. 架构优化建议");
    demonstrate_architecture_optimization(&dependency_report);
    println!();

    // 9. 实时依赖监控
    println!("📡 9. 实时依赖监控");
    demonstrate_dependency_monitoring(&analyzer).await?;
    println!();

    println!("✅ 深度依赖分析演示完成");
    println!("🎉 关键特性:");
    println!("   - 全面依赖关系分析");
    println!("   - 循环依赖自动检测");
    println!("   - 关键路径识别");
    println!("   - 迁移影响评估");
    println!("   - 架构优化建议");
    println!("   - 可视化数据生成");
    println!("   - 实时依赖监控");

    Ok(())
}

/// 演示循环依赖检测
fn demonstrate_circular_dependency_detection(report: &DependencyAnalysisReport) -> Result<(), Box<dyn std::error::Error>> {
    println!("   循环依赖检测结果:");

    if report.circular_dependencies.is_empty() {
        println!("   ✅ 未发现循环依赖");
    } else {
        println!("   🚨 发现 {} 个循环依赖:", report.circular_dependencies.len());

        for (i, cd) in report.circular_dependencies.iter().enumerate() {
            let severity_text = match cd.severity {
                CircularDependencySeverity::High => "高",
                CircularDependencySeverity::Medium => "中",
                CircularDependencySeverity::Low => "低",
            };

            println!("     {}. 严重程度: {}", i + 1, severity_text);
            println!("        循环路径: {}", cd.cycle.join(" -> "));

            // 提供解决建议
            match cd.severity {
                CircularDependencySeverity::High => {
                    println!("        建议: 立即重构以消除循环依赖");
                }
                CircularDependencySeverity::Medium => {
                    println!("        建议: 计划重构，考虑引入事件驱动架构");
                }
                CircularDependencySeverity::Low => {
                    println!("        建议: 监控并考虑在下次重构时解决");
                }
            }
            println!();
        }
    }

    Ok(())
}

/// 演示关键路径分析
fn demonstrate_critical_path_analysis(report: &DependencyAnalysisReport) -> Result<(), Box<dyn std::error::Error>> {
    println!("   关键路径分析结果:");

    if report.critical_paths.is_empty() {
        println!("   📊 未识别到关键路径");
    } else {
        println!("   🎯 识别到 {} 个关键路径:", report.critical_paths.len());

        for (i, path) in report.critical_paths.iter().take(5).enumerate() {
            let type_text = match path.path_type {
                CriticalPathType::Core => "核心服务",
                CriticalPathType::Hub => "枢纽服务",
                CriticalPathType::Bridge => "桥接服务",
            };

            println!("     {}. {} - {}", i + 1, path.critical_service, type_text);
            println!("        影响分数: {} (被 {} 个服务依赖)",
                path.impact_score, path.dependents.len());

            if path.dependents.len() <= 3 {
                println!("        依赖服务: {:?}", path.dependents);
            } else {
                println!("        主要依赖服务: {:?}", &path.dependents[..3]);
                println!("        ... 及其他 {} 个服务", path.dependents.len() - 3);
            }

            // 运维建议
            match path.path_type {
                CriticalPathType::Core => {
                    println!("        运维建议: 最高优先级监控，确保高可用性");
                }
                CriticalPathType::Hub => {
                    println!("        运维建议: 重点关注性能，考虑水平扩展");
                }
                CriticalPathType::Bridge => {
                    println!("        运维建议: 监控连接状态，准备降级方案");
                }
            }
            println!();
        }
    }

    Ok(())
}

/// 演示迁移影响分析
async fn demonstrate_migration_impact_analysis(analyzer: &DependencyAnalyzer) -> Result<(), Box<dyn std::error::Error>> {
    println!("   迁移影响分析:");

    // 分析几个关键服务的迁移影响
    let critical_services = vec![
        "authentication-service",
        "im-service",
        "contact-service",
        "group-service"
    ];

    for service in critical_services {
        println!("   📊 分析服务: {}", service);

        let impact_analysis = analyzer.analyze_migration_impact(service);

        let risk_level_text = match impact_analysis.risk_level {
            RiskLevel::Critical => "关键",
            RiskLevel::High => "高",
            RiskLevel::Medium => "中",
            RiskLevel::Low => "低",
        };

        println!("     风险等级: {}", risk_level_text);
        println!("     直接依赖: {} 个服务", impact_analysis.direct_dependencies.len());
        println!("     被依赖: {} 个服务", impact_analysis.dependents.len());
        println!("     影响范围: {} 个服务", impact_analysis.impact_scope.len());
        println!("     预估停机: {:?}", impact_analysis.estimated_downtime);
        println!("     推荐策略: {}", impact_analysis.recommended_strategy);

        if !impact_analysis.direct_dependencies.is_empty() {
            println!("     依赖详情:");
            for dep in &impact_analysis.direct_dependencies {
                println!("       - {}", dep);
            }
        }
        println!();
    }

    Ok(())
}

/// 演示依赖图可视化数据生成
fn demonstrate_dependency_graph_generation(analyzer: &DependencyAnalyzer) -> Result<(), Box<dyn std::error::Error>> {
    println!("   生成依赖图可视化数据...");

    let graph_data = analyzer.generate_dependency_graph_data();

    println!("   📈 图数据统计:");
    println!("     节点数: {}", graph_data.nodes.len());
    println!("     边数: {}", graph_data.edges.len());
    println!();

    println!("   🎯 节点详情 (前5个):");
    for (i, node) in graph_data.nodes.iter().take(5).enumerate() {
        println!("     {}. {} (层级: {}, 依赖数: {})",
            i + 1, node.label, node.level, node.dependency_count);
    }
    println!();

    println!("   🔗 边详情 (前5个):");
    for (i, edge) in graph_data.edges.iter().take(5).enumerate() {
        println!("     {}. {} -> {}", i + 1, edge.from, edge.to);
    }
    println!();

    // 生成简单的可视化描述
    println!("   🎨 可视化建议:");
    println!("     - 使用不同颜色表示服务层级");
    println!("     - 节点大小表示依赖数量");
    println!("     - 箭头粗细表示依赖强度");
    println!("     - 关键路径使用突出显示");
    println!();

    // 生成 Graphviz DOT 格式描述
    println!("   📐 Graphviz DOT 格式 (片段):");
    println!("     digraph ServiceDependencies {{");
    println!("       rankdir=LR;");
    println!("       node [shape=box];");

    for edge in graph_data.edges.iter().take(3) {
        println!("       \"{}\" -> \"{}\";", edge.from, edge.to);
    }
    println!("       // ... 更多边");
    println!("     }}");
    println!();

    Ok(())
}

/// 演示架构优化建议
fn demonstrate_architecture_optimization(report: &DependencyAnalysisReport) {
    println!("   基于依赖分析的架构优化建议:");

    // 统计信息
    let total_dependencies: usize = report.dependency_graph.values().map(|deps| deps.len()).sum();
    let avg_dependencies = total_dependencies as f64 / report.total_services as f64;
    let max_dependencies = report.dependency_graph.values().map(|deps| deps.len()).max().unwrap_or(0);

    println!("   📊 当前架构指标:");
    println!("     - 平均依赖数: {:.1}", avg_dependencies);
    println!("     - 最大依赖数: {}", max_dependencies);
    println!("     - 依赖层级数: {:?}", report.dependency_levels.values().max());
    println!();

    // 架构健康度评估
    let health_score = calculate_architecture_health_score(report);
    println!("   🏥 架构健康度评分: {:.1}/100", health_score);

    if health_score >= 80.0 {
        println!("   ✅ 架构状态: 健康");
    } else if health_score >= 60.0 {
        println!("   ⚠️  架构状态: 需要关注");
    } else {
        println!("   🚨 架构状态: 需要优化");
    }
    println!();

    // 具体建议
    println!("   💡 优化建议:");

    if avg_dependencies > 2.5 {
        println!("     🔧 降低耦合度:");
        println!("       - 考虑服务拆分");
        println!("       - 引入API网关");
        println!("       - 使用事件驱动架构");
        println!();
    }

    if max_dependencies > 4 {
        println!("     🎯 处理重依赖服务:");
        println!("       - 识别过度复杂的服务");
        println!("       - 实施服务重构");
        println!("       - 考虑功能分离");
        println!();
    }

    if !report.isolated_services.is_empty() {
        println!("     🏝️  处理孤立服务:");
        println!("       - 评估服务必要性");
        println!("       - 考虑服务整合");
        println!("       - 寻找集成机会");
        println!();
    }

    if !report.circular_dependencies.is_empty() {
        println!("     🔄 解决循环依赖:");
        println!("       - 重构服务依赖关系");
        println!("       - 引入依赖注入");
        println!("       - 使用异步通信");
        println!();
    }

    println!("     📈 长期改进:");
    println!("       - 建立架构治理流程");
    println!("       - 定期进行依赖分析");
    println!("       - 实施渐进式重构");
    println!("       - 建立服务目录和文档");
}

/// 计算架构健康度评分
fn calculate_architecture_health_score(report: &DependencyAnalysisReport) -> f64 {
    let mut score = 100.0;

    // 循环依赖扣分
    score -= report.circular_dependencies.len() as f64 * 15.0;

    // 孤立服务扣分
    score -= report.isolated_services.len() as f64 * 5.0;

    // 平均依赖数扣分
    let avg_dependencies: f64 = report.dependency_graph.values().map(|deps| deps.len()).sum::<usize>() as f64 / report.total_services as f64;
    if avg_dependencies > 2.0 {
        score -= (avg_dependencies - 2.0) * 10.0;
    }

    // 最大依赖深度扣分
    if let Some(&max_level) = report.dependency_levels.values().max() {
        if max_level > 4 {
            score -= (max_level - 4) as f64 * 8.0;
        }
    }

    score.max(0.0).min(100.0)
}

/// 演示依赖监控
async fn demonstrate_dependency_monitoring(analyzer: &DependencyAnalyzer) -> Result<(), Box<dyn std::error::Error>> {
    println!("   模拟实时依赖监控...");

    // 模拟监控过程
    for i in 1..=5 {
        tokio::time::sleep(Duration::from_millis(300)).await;

        let report = analyzer.analyze_dependencies();

        // 模拟不同的监控指标
        let health_score = calculate_architecture_health_score(&report);
        let risk_count = report.circular_dependencies.len();

        println!("     监控点 {}: 健康度 {:.1}%, 风险数 {} (状态: {})",
            i, health_score, risk_count,
            if health_score >= 80.0 && risk_count == 0 { "✅ 健康" }
            else if health_score >= 60.0 { "⚠️ 警告" }
            else { "🚨 异常" });

        // 模拟发现问题时的响应
        if i == 3 {
            println!("       🔔 检测到依赖变化，触发分析...");
            let critical_services = report.critical_paths.iter()
                .take(2)
                .map(|p| p.critical_service.clone())
                .collect::<Vec<_>>();

            if !critical_services.is_empty() {
                println!("       📊 关键服务状态检查: {:?}", critical_services);
            }
        }
    }

    println!();
    println!("   📊 监控总结:");
    println!("     - 监控频率: 300ms");
    println!("     - 健康度评估: ✅ 支持");
    println!("     - 异常检测: ✅ 支持");
    println!("     - 自动告警: ✅ 支持");
    println!("     - 趋势分析: ✅ 支持");

    Ok(())
}