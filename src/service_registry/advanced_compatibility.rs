//! ServiceRegistry 高级兼容性处理工具
//!
//! 提供智能兼容性分析、风险评估和迁移建议功能

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::{
    compatibility::{
        CompatibilityChecker, CompatibilityConfig, CompatibilityHandler, CompatibilityIssue,
        CompatibilityIssueType, CompatibilityLevel, IssueSeverity, ServiceVersion,
    },
    error::ServiceError,
    migration::{MigrationRisk, MigrationRiskType, MigrationStrategy},
    ServiceRegistry,
};
use crate::core::config::Config;

/// 高级兼容性分析器
pub struct AdvancedCompatibilityAnalyzer {
    registry: Arc<ServiceRegistry>,
    checker: CompatibilityChecker,
}

impl AdvancedCompatibilityAnalyzer {
    /// 创建新的高级兼容性分析器
    pub fn new(registry: Arc<ServiceRegistry>, config: CompatibilityConfig) -> Self {
        let checker = CompatibilityChecker::new(config);
        Self { registry, checker }
    }

    /// 执行全面的兼容性分析
    pub fn analyze_compatibility(&self, services: &[String]) -> CompatibilityAnalysisReport {
        let mut service_analysis = HashMap::new();

        // 分析每个服务的兼容性
        for service_name in services {
            let analysis = self.analyze_service_compatibility(service_name);
            service_analysis.insert(service_name.clone(), analysis);
        }

        // 检查跨服务依赖
        let cross_service_dependencies = self.analyze_cross_service_dependencies(services);

        // 识别全局问题
        let global_issues = self.identify_global_issues(&service_analysis, services);

        // 生成迁移建议
        let recommendations = self.generate_migration_recommendations(
            &service_analysis,
            &cross_service_dependencies,
            &global_issues,
        );

        CompatibilityAnalysisReport {
            total_services: services.len(),
            service_analysis,
            cross_service_dependencies,
            global_issues,
            recommendations,
            generated_at: std::time::SystemTime::now(),
        }
    }

    /// 分析单个服务的兼容性
    fn analyze_service_compatibility(&self, service_name: &str) -> ServiceCompatibilityAnalysis {
        let mut issues = Vec::new();
        let mut risks = Vec::new();

        // 获取服务信息
        if let Some(service_info) = self.registry.get_service_info(service_name) {
            // 检查服务版本兼容性
            let current_version = ServiceVersion::from_string(&service_info.version)
                .unwrap_or_else(|_| {
                    ServiceVersion::new(1, 0, 0) // 默认版本
                });

            // 模拟兼容性检查（实际应该与目标版本比较）
            let compatibility_result = self
                .checker
                .check_service_compatibility(service_name, &current_version, &self.registry)
                .unwrap_or_else(|_| {
                    // 如果检查失败，创建一个默认结果
                    crate::service_registry::compatibility::CompatibilityResult {
                        is_compatible: true,
                        compatibility_level: CompatibilityLevel::Full,
                        issues: vec![],
                        recommendations: vec![],
                    }
                });

            // 分析问题类型
            for issue in &compatibility_result.issues {
                match &issue.issue_type {
                    CompatibilityIssueType::VersionMismatch => {
                        risks.push(ServiceRisk {
                            risk_type: ServiceRiskType::VersionConflict,
                            severity: issue.severity.clone(),
                            description: issue.description.clone(),
                            impact: "可能影响与其他服务的通信".to_string(),
                            mitigation: "考虑版本升级或适配器模式".to_string(),
                        });
                    }
                    CompatibilityIssueType::ApiChange => {
                        risks.push(ServiceRisk {
                            risk_type: ServiceRiskType::ApiIncompatibility,
                            severity: issue.severity.clone(),
                            description: issue.description.clone(),
                            impact: "API 调用可能失败".to_string(),
                            mitigation: "更新客户端代码或使用兼容层".to_string(),
                        });
                    }
                    CompatibilityIssueType::DependencyMissing => {
                        risks.push(ServiceRisk {
                            risk_type: ServiceRiskType::DependencyIssue,
                            severity: issue.severity.clone(),
                            description: issue.description.clone(),
                            impact: "服务可能无法启动".to_string(),
                            mitigation: "确保依赖服务可用".to_string(),
                        });
                    }
                    _ => {}
                }
            }

            ServiceCompatibilityAnalysis {
                service_name: service_name.to_string(),
                current_version,
                compatibility_level: compatibility_result.compatibility_level,
                issues,
                risks,
                status: ServiceStatus::Active,
                dependencies: self.get_service_dependencies(service_name),
            }
        } else {
            // 服务未找到
            ServiceCompatibilityAnalysis {
                service_name: service_name.to_string(),
                current_version: ServiceVersion::new(0, 0, 0),
                compatibility_level: CompatibilityLevel::Incompatible,
                issues: vec![CompatibilityIssue {
                    issue_type: CompatibilityIssueType::DependencyMissing,
                    severity: IssueSeverity::Critical,
                    description: format!("Service '{}' not found in registry", service_name),
                    affected_services: vec![service_name.to_string()],
                }],
                risks: vec![ServiceRisk {
                    risk_type: ServiceRiskType::MissingService,
                    severity: IssueSeverity::Critical,
                    description: "Service is missing from registry".to_string(),
                    impact: "服务完全不可用".to_string(),
                    mitigation: "注册服务或检查服务名称".to_string(),
                }],
                status: ServiceStatus::NotFound,
                dependencies: Vec::new(),
            }
        }
    }

    /// 分析跨服务依赖
    fn analyze_cross_service_dependencies(
        &self,
        services: &[String],
    ) -> Vec<CrossServiceDependency> {
        let mut dependencies = Vec::new();
        let service_set: HashSet<_> = services.iter().collect();

        for service_name in services {
            let deps = self.get_service_dependencies(service_name);
            for dep in deps {
                if service_set.contains(&dep) {
                    dependencies.push(CrossServiceDependency {
                        from_service: service_name.clone(),
                        to_service: dep.clone(),
                        dependency_type: DependencyType::ServiceCall,
                        criticality: DependencyCriticality::High,
                    });
                }
            }
        }

        dependencies
    }

    /// 识别全局问题
    fn identify_global_issues(
        &self,
        service_analysis: &HashMap<String, ServiceCompatibilityAnalysis>,
        services: &[String],
    ) -> Vec<GlobalIssue> {
        let mut global_issues = Vec::new();

        // 检查版本一致性
        let mut versions: HashMap<String, usize> = HashMap::new();
        for analysis in service_analysis.values() {
            let version_str = analysis.current_version.to_string();
            *versions.entry(version_str).or_insert(0) += 1;
        }

        if versions.len() > 1 {
            global_issues.push(GlobalIssue {
                issue_type: GlobalIssueType::VersionInconsistency,
                severity: IssueSeverity::Warning,
                description: format!("发现 {} 个不同的服务版本", versions.len()),
                affected_services: services.to_vec(),
                impact: "可能导致兼容性问题".to_string(),
                resolution: "考虑统一服务版本".to_string(),
            });
        }

        // 检查关键服务缺失
        let critical_services = ["authentication-service", "im-service"];
        for critical in &critical_services {
            if !services.contains(&critical.to_string()) {
                global_issues.push(GlobalIssue {
                    issue_type: GlobalIssueType::MissingCriticalService,
                    severity: IssueSeverity::Critical,
                    description: format!("Critical service '{}' is missing", critical),
                    affected_services: services.to_vec(),
                    impact: "系统核心功能可能受影响".to_string(),
                    resolution: "确保所有关键服务都已注册".to_string(),
                });
            }
        }

        global_issues
    }

    /// 生成迁移建议
    fn generate_migration_recommendations(
        &self,
        service_analysis: &HashMap<String, ServiceCompatibilityAnalysis>,
        cross_service_deps: &[CrossServiceDependency],
        global_issues: &[GlobalIssue],
    ) -> Vec<MigrationRecommendation> {
        let mut recommendations = Vec::new();

        // 基于服务分析生成建议
        for analysis in service_analysis.values() {
            if !analysis.issues.is_empty() {
                recommendations.push(MigrationRecommendation {
                    category: RecommendationCategory::ServiceSpecific,
                    priority: RecommendationPriority::High,
                    title: format!("解决 {} 的兼容性问题", analysis.service_name),
                    description: format!(
                        "服务 {} 存在 {} 个兼容性问题",
                        analysis.service_name,
                        analysis.issues.len()
                    ),
                    actions: vec![
                        "检查服务版本".to_string(),
                        "验证依赖关系".to_string(),
                        "测试 API 兼容性".to_string(),
                    ],
                    estimated_effort: "中等".to_string(),
                });
            }
        }

        // 基于依赖关系生成建议
        if !cross_service_deps.is_empty() {
            recommendations.push(MigrationRecommendation {
                category: RecommendationCategory::DependencyManagement,
                priority: RecommendationPriority::Medium,
                title: "管理跨服务依赖".to_string(),
                description: format!("发现 {} 个跨服务依赖关系", cross_service_deps.len()),
                actions: vec![
                    "按依赖顺序迁移".to_string(),
                    "考虑服务解耦".to_string(),
                    "实施熔断机制".to_string(),
                ],
                estimated_effort: "高".to_string(),
            });
        }

        // 基于全局问题生成建议
        for issue in global_issues {
            recommendations.push(MigrationRecommendation {
                category: RecommendationCategory::GlobalOptimization,
                priority: match issue.severity {
                    IssueSeverity::Critical => RecommendationPriority::Critical,
                    IssueSeverity::Error => RecommendationPriority::High,
                    IssueSeverity::Warning => RecommendationPriority::Medium,
                    IssueSeverity::Info => RecommendationPriority::Low,
                },
                title: format!("解决全局问题: {:?}", issue.issue_type),
                description: issue.description.clone(),
                actions: vec![issue.resolution.clone()],
                estimated_effort: "视具体情况而定".to_string(),
            });
        }

        // 通用最佳实践建议
        recommendations.push(MigrationRecommendation {
            category: RecommendationCategory::BestPractice,
            priority: RecommendationPriority::Medium,
            title: "遵循迁移最佳实践".to_string(),
            description: "确保迁移过程的安全性和可靠性".to_string(),
            actions: vec![
                "制定回滚计划".to_string(),
                "分阶段迁移".to_string(),
                "监控迁移过程".to_string(),
                "备份关键数据".to_string(),
            ],
            estimated_effort: "低".to_string(),
        });

        recommendations
    }

    /// 获取服务依赖
    fn get_service_dependencies(&self, service_name: &str) -> Vec<String> {
        // 这里应该实现实际的依赖检测逻辑
        // 目前返回模拟数据
        match service_name {
            "im-service" => vec!["authentication-service".to_string()],
            "contact-service" => vec!["authentication-service".to_string()],
            "group-service" => vec![
                "authentication-service".to_string(),
                "im-service".to_string(),
            ],
            _ => Vec::new(),
        }
    }

    /// 推荐最佳迁移策略
    pub fn recommend_migration_strategy(
        &self,
        services: &[String],
        config: &Config,
    ) -> RecommendedStrategy {
        let analysis = self.analyze_compatibility(services);

        // 分析服务数量和复杂度
        let service_count = services.len();
        let has_critical_issues = analysis.global_issues.iter().any(|issue| {
            matches!(
                issue.severity,
                IssueSeverity::Critical | IssueSeverity::Error
            )
        });
        let has_many_dependencies = analysis.cross_service_dependencies.len() > service_count / 2;

        let strategy = match (service_count, has_critical_issues, has_many_dependencies) {
            (0..=5, false, false) => (
                MigrationStrategy::Immediate,
                "服务数量少，无关键问题，建议立即迁移".to_string(),
            ),
            (6..=20, false, false) => (
                MigrationStrategy::Gradual {
                    batch_size: 5,
                    delay_between_batches: std::time::Duration::from_secs(30),
                },
                "服务数量适中，建议分批渐进迁移".to_string(),
            ),
            (_, true, _) => (
                MigrationStrategy::Canary {
                    canary_services: vec!["authentication-service".to_string()],
                },
                "存在关键问题，建议金丝雀发布".to_string(),
            ),
            (_, _, true) => (
                MigrationStrategy::BlueGreen {
                    validate_before_switch: true,
                },
                "依赖关系复杂，建议蓝绿部署".to_string(),
            ),
            _ => (
                MigrationStrategy::Gradual {
                    batch_size: 3,
                    delay_between_batches: std::time::Duration::from_secs(60),
                },
                "复杂场景，建议保守的渐进迁移".to_string(),
            ),
        };

        let (strategy, reason) = strategy;

        RecommendedStrategy {
            strategy: strategy.clone(),
            reason,
            confidence: self.calculate_strategy_confidence(&analysis),
            estimated_duration: self.estimate_migration_duration(services, &strategy),
        }
    }

    /// 计算策略置信度
    fn calculate_strategy_confidence(&self, analysis: &CompatibilityAnalysisReport) -> f64 {
        let mut confidence = 0.8; // 基础置信度

        // 根据问题数量调整
        let total_issues = analysis
            .service_analysis
            .values()
            .map(|s| s.issues.len())
            .sum::<usize>();
        confidence -= (total_issues as f64 * 0.05).min(0.3);

        // 根据依赖复杂度调整
        let dependency_ratio =
            analysis.cross_service_dependencies.len() as f64 / analysis.total_services as f64;
        confidence -= (dependency_ratio * 0.1).min(0.2);

        confidence.max(0.1).min(0.95)
    }

    /// 估算迁移时间
    fn estimate_migration_duration(
        &self,
        services: &[String],
        strategy: &MigrationStrategy,
    ) -> std::time::Duration {
        let base_time_per_service = std::time::Duration::from_secs(5); // 基础时间估算

        match strategy {
            MigrationStrategy::Immediate => base_time_per_service * services.len() as u32,
            MigrationStrategy::Gradual {
                batch_size,
                delay_between_batches,
            } => {
                let batch_count = (services.len() + batch_size - 1) / batch_size;
                let total_delay =
                    delay_between_batches.saturating_mul(batch_count.saturating_sub(1) as u32);
                total_delay + base_time_per_service * services.len() as u32
            }
            MigrationStrategy::Canary { .. } => {
                // 金丝雀部署需要额外时间进行验证
                (base_time_per_service * services.len() as u32) * 2
            }
            MigrationStrategy::BlueGreen { .. } => {
                // 蓝绿部署需要额外时间进行验证
                (base_time_per_service * services.len() as u32) * 2
            }
        }
    }
}

/// 兼容性分析报告
#[derive(Debug, Clone)]
pub struct CompatibilityAnalysisReport {
    /// 总服务数
    pub total_services: usize,
    /// 服务分析结果
    pub service_analysis: HashMap<String, ServiceCompatibilityAnalysis>,
    /// 跨服务依赖
    pub cross_service_dependencies: Vec<CrossServiceDependency>,
    /// 全局问题
    pub global_issues: Vec<GlobalIssue>,
    /// 迁移建议
    pub recommendations: Vec<MigrationRecommendation>,
    /// 生成时间
    pub generated_at: std::time::SystemTime,
}

/// 服务兼容性分析
#[derive(Debug, Clone)]
pub struct ServiceCompatibilityAnalysis {
    /// 服务名称
    pub service_name: String,
    /// 当前版本
    pub current_version: ServiceVersion,
    /// 兼容性级别
    pub compatibility_level: CompatibilityLevel,
    /// 问题列表
    pub issues: Vec<CompatibilityIssue>,
    /// 风险列表
    pub risks: Vec<ServiceRisk>,
    /// 服务状态
    pub status: ServiceStatus,
    /// 依赖服务
    pub dependencies: Vec<String>,
}

/// 服务风险
#[derive(Debug, Clone)]
pub struct ServiceRisk {
    /// 风险类型
    pub risk_type: ServiceRiskType,
    /// 严重程度
    pub severity: IssueSeverity,
    /// 描述
    pub description: String,
    /// 影响
    pub impact: String,
    /// 缓解措施
    pub mitigation: String,
}

/// 服务风险类型
#[derive(Debug, Clone)]
pub enum ServiceRiskType {
    /// 版本冲突
    VersionConflict,
    /// API 不兼容
    ApiIncompatibility,
    /// 依赖问题
    DependencyIssue,
    /// 服务缺失
    MissingService,
    /// 配置问题
    ConfigurationIssue,
}

/// 服务状态
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    /// 活跃
    Active,
    /// 未找到
    NotFound,
    /// 错误
    Error,
    /// 维护中
    Maintenance,
}

/// 跨服务依赖
#[derive(Debug, Clone)]
pub struct CrossServiceDependency {
    /// 依赖来源服务
    pub from_service: String,
    /// 依赖目标服务
    pub to_service: String,
    /// 依赖类型
    pub dependency_type: DependencyType,
    /// 关键程度
    pub criticality: DependencyCriticality,
}

/// 依赖类型
#[derive(Debug, Clone)]
pub enum DependencyType {
    /// 服务调用
    ServiceCall,
    /// 数据依赖
    DataDependency,
    /// 配置依赖
    ConfigurationDependency,
}

/// 依赖关键程度
#[derive(Debug, Clone)]
pub enum DependencyCriticality {
    /// 高
    High,
    /// 中
    Medium,
    /// 低
    Low,
}

/// 全局问题
#[derive(Debug, Clone)]
pub struct GlobalIssue {
    /// 问题类型
    pub issue_type: GlobalIssueType,
    /// 严重程度
    pub severity: IssueSeverity,
    /// 描述
    pub description: String,
    /// 受影响的服务
    pub affected_services: Vec<String>,
    /// 影响
    pub impact: String,
    /// 解决方案
    pub resolution: String,
}

/// 全局问题类型
#[derive(Debug, Clone)]
pub enum GlobalIssueType {
    /// 版本不一致
    VersionInconsistency,
    /// 缺失关键服务
    MissingCriticalService,
    /// 配置冲突
    ConfigurationConflict,
    /// 性能问题
    PerformanceIssue,
    /// 安全问题
    SecurityIssue,
}

/// 迁移建议
#[derive(Debug, Clone)]
pub struct MigrationRecommendation {
    /// 建议类别
    pub category: RecommendationCategory,
    /// 优先级
    pub priority: RecommendationPriority,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 行动项
    pub actions: Vec<String>,
    /// 预估工作量
    pub estimated_effort: String,
}

/// 建议类别
#[derive(Debug, Clone)]
pub enum RecommendationCategory {
    /// 服务特定
    ServiceSpecific,
    /// 依赖管理
    DependencyManagement,
    /// 全局优化
    GlobalOptimization,
    /// 最佳实践
    BestPractice,
}

/// 建议优先级
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    /// 关键
    Critical,
    /// 高
    High,
    /// 中
    Medium,
    /// 低
    Low,
}

/// 推荐策略
#[derive(Debug, Clone)]
pub struct RecommendedStrategy {
    /// 推荐策略
    pub strategy: MigrationStrategy,
    /// 推荐理由
    pub reason: String,
    /// 置信度
    pub confidence: f64,
    /// 预估时间
    pub estimated_duration: std::time::Duration,
}

impl CompatibilityAnalysisReport {
    /// 打印分析报告
    pub fn print(&self) {
        println!("📊 ServiceRegistry 兼容性分析报告");
        println!("================================");
        println!("分析时间: {:?}", self.generated_at);
        println!("总服务数: {}", self.total_services);
        println!();

        // 服务分析摘要
        let compatible_count = self
            .service_analysis
            .values()
            .filter(|s| matches!(s.compatibility_level, CompatibilityLevel::Full))
            .count();
        let partial_count = self
            .service_analysis
            .values()
            .filter(|s| matches!(s.compatibility_level, CompatibilityLevel::Partial))
            .count();
        let incompatible_count = self
            .service_analysis
            .values()
            .filter(|s| matches!(s.compatibility_level, CompatibilityLevel::Incompatible))
            .count();

        println!("📈 兼容性摘要:");
        println!("  ✅ 完全兼容: {} 个服务", compatible_count);
        println!("  ⚠️  部分兼容: {} 个服务", partial_count);
        println!("  ❌ 不兼容: {} 个服务", incompatible_count);
        println!();

        // 跨服务依赖
        if !self.cross_service_dependencies.is_empty() {
            println!(
                "🔗 跨服务依赖 ({} 个):",
                self.cross_service_dependencies.len()
            );
            for dep in &self.cross_service_dependencies {
                let criticality_icon = match dep.criticality {
                    DependencyCriticality::High => "🔴",
                    DependencyCriticality::Medium => "🟡",
                    DependencyCriticality::Low => "🟢",
                };
                println!(
                    "  {} {} -> {} ({:?})",
                    criticality_icon, dep.from_service, dep.to_service, dep.dependency_type
                );
            }
            println!();
        }

        // 全局问题
        if !self.global_issues.is_empty() {
            println!("⚠️  全局问题 ({} 个):", self.global_issues.len());
            for issue in &self.global_issues {
                let severity_icon = match issue.severity {
                    IssueSeverity::Critical => "🔴",
                    IssueSeverity::Error => "❌",
                    IssueSeverity::Warning => "⚠️",
                    IssueSeverity::Info => "ℹ️",
                };
                println!("  {} {:?}", severity_icon, issue.issue_type);
                println!("    {}", issue.description);
                println!("    影响: {}", issue.impact);
                println!("    解决: {}", issue.resolution);
                println!();
            }
        }

        // 高优先级建议
        let high_priority_recommendations: Vec<_> = self
            .recommendations
            .iter()
            .filter(|r| {
                matches!(
                    r.priority,
                    RecommendationPriority::Critical | RecommendationPriority::High
                )
            })
            .collect();

        if !high_priority_recommendations.is_empty() {
            println!(
                "🚨 高优先级建议 ({} 个):",
                high_priority_recommendations.len()
            );
            for rec in high_priority_recommendations {
                let priority_icon = match rec.priority {
                    RecommendationPriority::Critical => "🔴",
                    RecommendationPriority::High => "🟠",
                    _ => "🟡",
                };
                println!("  {} {}", priority_icon, rec.title);
                println!("    {}", rec.description);
                println!("    工作量: {}", rec.estimated_effort);
                if !rec.actions.is_empty() {
                    println!("    行动项:");
                    for action in &rec.actions {
                        println!("      - {}", action);
                    }
                }
                println!();
            }
        }

        // 所有建议摘要
        println!("💡 建议摘要 (共 {} 个):", self.recommendations.len());
        let mut category_counts = HashMap::new();
        for rec in &self.recommendations {
            *category_counts
                .entry(format!("{:?}", rec.category))
                .or_insert(0) += 1;
        }
        for (category, count) in category_counts {
            println!("  {}: {} 个建议", category, count);
        }
    }
}
