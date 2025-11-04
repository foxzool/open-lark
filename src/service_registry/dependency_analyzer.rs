//! ServiceRegistry 服务依赖分析器
//!
//! 提供深度服务依赖分析、循环依赖检测和优化建议功能

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::{
    error::ServiceError,
    registry::ServiceRegistry,
    service::{ServiceInfo, ServiceStatus},
};

/// 服务依赖分析器
pub struct DependencyAnalyzer {
    registry: Arc<ServiceRegistry>,
}

impl DependencyAnalyzer {
    /// 创建新的依赖分析器
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        Self { registry }
    }

    /// 执行全面的依赖分析
    pub fn analyze_dependencies(&self) -> DependencyAnalysisReport {
        let services = self.registry.discover_services();
        let mut service_info = HashMap::new();
        let mut dependency_graph = HashMap::new();

        // 收集服务信息
        for service_name in &services {
            if let Some(info) = self.registry.get_service_info(service_name) {
                service_info.insert(service_name.to_string(), info.clone());
            }
        }

        // 构建依赖图
        for service_name in &services {
            let dependencies = self.get_service_dependencies(service_name);
            dependency_graph.insert(service_name.to_string(), dependencies);
        }

        // 分析依赖层级
        let dependency_levels = self.calculate_dependency_levels(&dependency_graph);

        // 检测循环依赖
        let circular_dependencies = self.detect_circular_dependencies(&dependency_graph);

        // 分析关键路径
        let critical_paths = self.analyze_critical_paths(&dependency_graph);

        // 识别孤立服务
        let isolated_services = self.identify_isolated_services(&dependency_graph);

        // 生成优化建议
        let recommendations = self.generate_dependency_recommendations(
            &dependency_graph,
            &circular_dependencies,
            &isolated_services,
        );

        DependencyAnalysisReport {
            total_services: services.len(),
            service_info,
            dependency_graph,
            dependency_levels,
            circular_dependencies,
            critical_paths,
            isolated_services,
            recommendations,
            analysis_time: std::time::SystemTime::now(),
        }
    }

    /// 获取服务依赖（基于服务类型和常见模式）
    fn get_service_dependencies(&self, service_name: &str) -> Vec<String> {
        // 基于服务名称推断依赖关系
        match service_name {
            // 核心服务依赖
            "authentication-service" => vec![], // 无依赖，基础服务

            // 业务服务依赖
            "im-service" => vec!["authentication-service".to_string()],
            "contact-service" => vec!["authentication-service".to_string()],
            "group-service" => vec![
                "authentication-service".to_string(),
                "im-service".to_string(),
                "contact-service".to_string(),
            ],
            "search-service" => vec![
                "authentication-service".to_string(),
                "im-service".to_string(),
                "contact-service".to_string(),
            ],

            // 高级服务依赖
            "calendar-service" => vec![
                "authentication-service".to_string(),
                "contact-service".to_string(),
            ],
            "approval-service" => vec![
                "authentication-service".to_string(),
                "im-service".to_string(),
                "contact-service".to_string(),
            ],
            "drive-service" => vec![
                "authentication-service".to_string(),
                "contact-service".to_string(),
            ],
            "wiki-service" => vec![
                "authentication-service".to_string(),
                "contact-service".to_string(),
            ],

            // AI 服务依赖
            "ai-service" => vec![
                "authentication-service".to_string(),
                "drive-service".to_string(),
            ],

            // 企业服务依赖
            "hr-service" => vec![
                "authentication-service".to_string(),
                "contact-service".to_string(),
                "approval-service".to_string(),
            ],
            "finance-service" => vec![
                "authentication-service".to_string(),
                "approval-service".to_string(),
            ],

            // 默认：只依赖认证服务
            _ => vec!["authentication-service".to_string()],
        }
    }

    /// 计算依赖层级
    fn calculate_dependency_levels(&self, dependency_graph: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
        let mut levels = HashMap::new();
        let mut visited = HashSet::new();

        for service in dependency_graph.keys() {
            self.calculate_service_level(service, dependency_graph, &mut levels, &mut visited);
        }

        levels
    }

    /// 递归计算服务层级
    fn calculate_service_level(
        &self,
        service: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
        levels: &mut HashMap<String, usize>,
        visited: &mut HashSet<String>,
    ) -> usize {
        if let Some(&level) = levels.get(service) {
            return level;
        }

        if visited.contains(service) {
            // 检测到循环依赖，分配一个层级
            return 0;
        }

        visited.insert(service.to_string());

        let empty_deps = vec![];
        let dependencies = dependency_graph.get(service).unwrap_or(&empty_deps);
        let max_dep_level = dependencies
            .iter()
            .map(|dep| self.calculate_service_level(dep, dependency_graph, levels, visited))
            .max()
            .unwrap_or(0);

        let level = max_dep_level + 1;
        levels.insert(service.to_string(), level);
        level
    }

    /// 检测循环依赖
    fn detect_circular_dependencies(&self, dependency_graph: &HashMap<String, Vec<String>>) -> Vec<CircularDependency> {
        let mut circular_deps = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for service in dependency_graph.keys() {
            if !visited.contains(service) {
                self.detect_circular_dependencies_dfs(
                    service,
                    dependency_graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut circular_deps,
                );
            }
        }

        circular_deps
    }

    /// 深度优先搜索检测循环依赖
    fn detect_circular_dependencies_dfs(
        &self,
        service: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        circular_deps: &mut Vec<CircularDependency>,
    ) {
        visited.insert(service.to_string());
        rec_stack.insert(service.to_string());
        path.push(service.to_string());

        if let Some(dependencies) = dependency_graph.get(service) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    self.detect_circular_dependencies_dfs(
                        dep,
                        dependency_graph,
                        visited,
                        rec_stack,
                        path,
                        circular_deps,
                    );
                } else if rec_stack.contains(dep) {
                    // 发现循环依赖
                    if let Some(start_index) = path.iter().position(|s| s == dep) {
                        let cycle = path[start_index..].to_vec();
                        circular_deps.push(CircularDependency {
                            cycle: cycle.clone(),
                            severity: self.assess_cycle_severity(&cycle),
                        });
                    }
                }
            }
        }

        rec_stack.remove(service);
        path.pop();
    }

    /// 评估循环依赖严重程度
    fn assess_cycle_severity(&self, cycle: &[String]) -> CircularDependencySeverity {
        match cycle.len() {
            1..=2 => CircularDependencySeverity::High,
            3..=4 => CircularDependencySeverity::Medium,
            _ => CircularDependencySeverity::Low,
        }
    }

    /// 分析关键路径
    fn analyze_critical_paths(&self, dependency_graph: &HashMap<String, Vec<String>>) -> Vec<CriticalPath> {
        let mut critical_paths = Vec::new();

        // 找出被最多服务依赖的关键服务
        let mut dependency_count = HashMap::new();
        for dependencies in dependency_graph.values() {
            for dep in dependencies {
                *dependency_count.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // 识别关键路径
        for (service, count) in dependency_count {
            if count >= 2 {
                let dependents = dependency_graph
                    .iter()
                    .filter_map(|(s, deps)| {
                        if deps.contains(&service) {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                critical_paths.push(CriticalPath {
                    critical_service: service.clone(),
                    dependents,
                    impact_score: count,
                    path_type: if service == "authentication-service" {
                        CriticalPathType::Core
                    } else if count >= 5 {
                        CriticalPathType::Hub
                    } else {
                        CriticalPathType::Bridge
                    },
                });
            }
        }

        critical_paths.sort_by(|a, b| b.impact_score.cmp(&a.impact_score));
        critical_paths
    }

    /// 识别孤立服务
    fn identify_isolated_services(&self, dependency_graph: &HashMap<String, Vec<String>>) -> Vec<String> {
        let mut dependent_services = HashSet::new();
        let mut dependent_on_others = HashSet::new();

        for (service, dependencies) in dependency_graph {
            if !dependencies.is_empty() {
                dependent_on_others.insert(service.clone());
                for dep in dependencies {
                    dependent_services.insert(dep.clone());
                }
            }
        }

        // 孤立服务：既不依赖其他服务，也不被其他服务依赖
        dependency_graph
            .keys()
            .filter(|service| {
                !dependent_on_others.contains(*service) && !dependent_services.contains(*service)
            })
            .cloned()
            .collect()
    }

    /// 生成依赖优化建议
    fn generate_dependency_recommendations(
        &self,
        dependency_graph: &HashMap<String, Vec<String>>,
        circular_dependencies: &[CircularDependency],
        isolated_services: &[String],
    ) -> Vec<DependencyRecommendation> {
        let mut recommendations = Vec::new();

        // 循环依赖建议
        if !circular_dependencies.is_empty() {
            recommendations.push(DependencyRecommendation {
                category: RecommendationCategory::DependencyIssue,
                priority: RecommendationPriority::Critical,
                title: "解决循环依赖".to_string(),
                description: format!("发现 {} 个循环依赖，需要立即解决", circular_dependencies.len()),
                actions: vec![
                    "重构服务架构以消除循环依赖".to_string(),
                    "引入依赖注入或事件驱动架构".to_string(),
                    "考虑将共同依赖提取为独立服务".to_string(),
                ],
                affected_services: circular_dependencies
                    .iter()
                    .flat_map(|cd| cd.cycle.clone())
                    .collect(),
            });
        }

        // 深度依赖链建议
        let max_depth = dependency_graph
            .keys()
            .map(|s| dependency_graph.get(s).unwrap_or(&vec![]).len())
            .max()
            .unwrap_or(0);

        if max_depth > 3 {
            recommendations.push(DependencyRecommendation {
                category: RecommendationCategory::ArchitectureOptimization,
                priority: RecommendationPriority::High,
                title: "优化深度依赖链".to_string(),
                description: format!("最大依赖深度为 {}，建议简化架构", max_depth),
                actions: vec![
                    "考虑服务拆分以减少依赖深度".to_string(),
                    "引入中间层服务".to_string(),
                    "使用异步通信减少直接依赖".to_string(),
                ],
                affected_services: dependency_graph
                    .iter()
                    .filter(|(_, deps)| deps.len() >= 3)
                    .map(|(s, _)| s.clone())
                    .collect(),
            });
        }

        // 孤立服务建议
        if !isolated_services.is_empty() {
            recommendations.push(DependencyRecommendation {
                category: RecommendationCategory::ServiceUtilization,
                priority: RecommendationPriority::Medium,
                title: "评估孤立服务".to_string(),
                description: format!("发现 {} 个孤立服务，建议评估其必要性", isolated_services.len()),
                actions: vec![
                    "检查孤立服务是否仍在使用".to_string(),
                    "考虑移除不再需要的孤立服务".to_string(),
                    "为有用的孤立服务寻找集成机会".to_string(),
                ],
                affected_services: isolated_services.to_vec(),
            });
        }

        // 架构优化建议
        let total_dependencies: usize = dependency_graph.values().map(|deps| deps.len()).sum();
        let avg_dependencies = total_dependencies as f64 / dependency_graph.len() as f64;

        if avg_dependencies > 2.5 {
            recommendations.push(DependencyRecommendation {
                category: RecommendationCategory::ArchitectureOptimization,
                priority: RecommendationPriority::Medium,
                title: "降低服务耦合度".to_string(),
                description: format!("平均依赖数量为 {:.1}，建议降低耦合度", avg_dependencies),
                actions: vec![
                    "实施服务解耦策略".to_string(),
                    "使用API网关减少直接依赖".to_string(),
                    "引入事件驱动架构".to_string(),
                    "考虑微服务拆分".to_string(),
                ],
                affected_services: dependency_graph
                    .iter()
                    .filter(|(_, deps)| deps.len() > 2)
                    .map(|(s, _)| s.clone())
                    .collect(),
            });
        }

        recommendations
    }

    /// 生成依赖图可视化数据
    pub fn generate_dependency_graph_data(&self) -> DependencyGraphData {
        let report = self.analyze_dependencies();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // 生成节点
        for (service_name, info) in &report.service_info {
            let level = report.dependency_levels.get(service_name).unwrap_or(&0);
            nodes.push(DependencyNode {
                id: service_name.clone(),
                label: service_name.clone(),
                level: *level,
                status: info.status.clone(),
                dependency_count: report.dependency_graph.get(service_name).unwrap_or(&vec![]).len(),
            });
        }

        // 生成边
        for (service, dependencies) in &report.dependency_graph {
            for dep in dependencies {
                edges.push(DependencyEdge {
                    from: service.clone(),
                    to: dep.clone(),
                    relationship: DependencyRelationship::DependsOn,
                });
            }
        }

        DependencyGraphData { nodes, edges }
    }

    /// 分析服务迁移影响
    pub fn analyze_migration_impact(&self, service: &str) -> MigrationImpactAnalysis {
        let report = self.analyze_dependencies();

        // 找出直接依赖
        let direct_dependencies = report.dependency_graph.get(service).cloned().unwrap_or_default();

        // 找出依赖此服务的服务
        let dependents: Vec<String> = report.dependency_graph
            .iter()
            .filter_map(|(s, deps)| {
                if deps.contains(&service.to_string()) {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        // 计算影响范围
        let impact_scope = self.calculate_impact_scope(service, &report.dependency_graph);

        // 评估风险等级
        let risk_level = self.assess_migration_risk(service, &direct_dependencies, &dependents);

        MigrationImpactAnalysis {
            target_service: service.to_string(),
            direct_dependencies,
            dependents: dependents.clone(),
            impact_scope,
            risk_level: risk_level.clone(),
            estimated_downtime: self.estimate_downtime(&risk_level, &dependents),
            recommended_strategy: self.recommend_migration_strategy(&risk_level, &dependents),
        }
    }

    /// 计算影响范围
    fn calculate_impact_scope(&self, service: &str, dependency_graph: &HashMap<String, Vec<String>>) -> Vec<String> {
        let mut affected = HashSet::new();
        let mut to_visit = vec![service.to_string()];

        while let Some(current) = to_visit.pop() {
            if affected.contains(&current) {
                continue;
            }
            affected.insert(current.clone());

            // 添加依赖此服务的服务
            for (s, deps) in dependency_graph {
                if deps.contains(&current) && !affected.contains(s) {
                    to_visit.push(s.clone());
                }
            }
        }

        affected.into_iter().filter(|s| s != service).collect()
    }

    /// 评估迁移风险
    fn assess_migration_risk(
        &self,
        service: &str,
        direct_dependencies: &[String],
        dependents: &[String],
    ) -> RiskLevel {
        let total_impact = direct_dependencies.len() + dependents.len();

        match (service, total_impact) {
            ("authentication-service", _) => RiskLevel::Critical,
            (_, 0) => RiskLevel::Low,
            (_, 1..=2) => RiskLevel::Medium,
            (_, 3..=5) => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    /// 估算停机时间
    fn estimate_downtime(&self, risk_level: &RiskLevel, dependents: &[String]) -> std::time::Duration {
        let base_time = match risk_level {
            RiskLevel::Critical => std::time::Duration::from_secs(300), // 5分钟
            RiskLevel::High => std::time::Duration::from_secs(180),     // 3分钟
            RiskLevel::Medium => std::time::Duration::from_secs(60),    // 1分钟
            RiskLevel::Low => std::time::Duration::from_secs(30),       // 30秒
        };

        // 根据依赖服务数量调整
        let multiplier = 1.0 + (dependents.len() as f64 * 0.1);
        base_time.mul_f32(multiplier as f32)
    }

    /// 推荐迁移策略
    fn recommend_migration_strategy(&self, risk_level: &RiskLevel, dependents: &[String]) -> String {
        match (risk_level, dependents.len()) {
            (RiskLevel::Critical, _) => "使用蓝绿部署，确保零停机".to_string(),
            (RiskLevel::High, _) => "使用金丝雀发布，逐步验证".to_string(),
            (RiskLevel::Medium, 1..=2) => "使用滚动更新，分批处理".to_string(),
            (RiskLevel::Medium, _) => "使用分阶段部署，控制影响范围".to_string(),
            (RiskLevel::Low, _) => "使用标准部署流程".to_string(),
        }
    }
}

/// 依赖分析报告
#[derive(Debug, Clone)]
pub struct DependencyAnalysisReport {
    /// 总服务数
    pub total_services: usize,
    /// 服务信息
    pub service_info: HashMap<String, ServiceInfo>,
    /// 依赖图
    pub dependency_graph: HashMap<String, Vec<String>>,
    /// 依赖层级
    pub dependency_levels: HashMap<String, usize>,
    /// 循环依赖
    pub circular_dependencies: Vec<CircularDependency>,
    /// 关键路径
    pub critical_paths: Vec<CriticalPath>,
    /// 孤立服务
    pub isolated_services: Vec<String>,
    /// 优化建议
    pub recommendations: Vec<DependencyRecommendation>,
    /// 分析时间
    pub analysis_time: std::time::SystemTime,
}

/// 循环依赖
#[derive(Debug, Clone)]
pub struct CircularDependency {
    /// 循环路径
    pub cycle: Vec<String>,
    /// 严重程度
    pub severity: CircularDependencySeverity,
}

/// 循环依赖严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum CircularDependencySeverity {
    /// 高严重性
    High,
    /// 中等严重性
    Medium,
    /// 低严重性
    Low,
}

/// 关键路径
#[derive(Debug, Clone)]
pub struct CriticalPath {
    /// 关键服务
    pub critical_service: String,
    /// 依赖此服务的服务
    pub dependents: Vec<String>,
    /// 影响分数
    pub impact_score: usize,
    /// 路径类型
    pub path_type: CriticalPathType,
}

/// 关键路径类型
#[derive(Debug, Clone)]
pub enum CriticalPathType {
    /// 核心服务
    Core,
    /// 枢纽服务
    Hub,
    /// 桥接服务
    Bridge,
}

/// 依赖建议
#[derive(Debug, Clone)]
pub struct DependencyRecommendation {
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
    /// 受影响的服务
    pub affected_services: Vec<String>,
}

/// 建议类别
#[derive(Debug, Clone)]
pub enum RecommendationCategory {
    /// 依赖问题
    DependencyIssue,
    /// 架构优化
    ArchitectureOptimization,
    /// 服务利用
    ServiceUtilization,
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

/// 依赖图数据
#[derive(Debug, Clone)]
pub struct DependencyGraphData {
    /// 节点
    pub nodes: Vec<DependencyNode>,
    /// 边
    pub edges: Vec<DependencyEdge>,
}

/// 依赖节点
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// 节点ID
    pub id: String,
    /// 标签
    pub label: String,
    /// 层级
    pub level: usize,
    /// 状态
    pub status: ServiceStatus,
    /// 依赖数量
    pub dependency_count: usize,
}

/// 依赖边
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    /// 源节点
    pub from: String,
    /// 目标节点
    pub to: String,
    /// 关系类型
    pub relationship: DependencyRelationship,
}

/// 依赖关系
#[derive(Debug, Clone)]
pub enum DependencyRelationship {
    /// 依赖于
    DependsOn,
    /// 被依赖
    DependencyOf,
}

/// 迁移影响分析
#[derive(Debug, Clone)]
pub struct MigrationImpactAnalysis {
    /// 目标服务
    pub target_service: String,
    /// 直接依赖
    pub direct_dependencies: Vec<String>,
    /// 依赖此服务的服务
    pub dependents: Vec<String>,
    /// 影响范围
    pub impact_scope: Vec<String>,
    /// 风险等级
    pub risk_level: RiskLevel,
    /// 预估停机时间
    pub estimated_downtime: std::time::Duration,
    /// 推荐策略
    pub recommended_strategy: String,
}

/// 风险等级
#[derive(Debug, Clone)]
pub enum RiskLevel {
    /// 关键
    Critical,
    /// 高
    High,
    /// 中
    Medium,
    /// 低
    Low,
}

impl DependencyAnalysisReport {
    /// 打印分析报告
    pub fn print(&self) {
        println!("🔗 ServiceRegistry 依赖分析报告");
        println!("==============================");
        println!("分析时间: {:?}", self.analysis_time);
        println!("总服务数: {}", self.total_services);
        println!();

        // 依赖层级摘要
        println!("📊 依赖层级分布:");
        let mut level_counts = HashMap::new();
        for level in self.dependency_levels.values() {
            *level_counts.entry(*level).or_insert(0) += 1;
        }
        let mut sorted_levels: Vec<_> = level_counts.iter().collect();
        sorted_levels.sort_by_key(|(level, _)| *level);
        for (level, count) in sorted_levels {
            println!("  层级 {}: {} 个服务", level, count);
        }
        println!();

        // 循环依赖
        if !self.circular_dependencies.is_empty() {
            println!("🔄 循环依赖 ({} 个):", self.circular_dependencies.len());
            for (i, cd) in self.circular_dependencies.iter().enumerate() {
                let severity_icon = match cd.severity {
                    CircularDependencySeverity::High => "🔴",
                    CircularDependencySeverity::Medium => "🟡",
                    CircularDependencySeverity::Low => "🟢",
                };
                println!("  {} 循环 {}: {:?} -> {}",
                    severity_icon, i + 1, cd.cycle, cd.cycle.get(0).unwrap_or(&"<unknown>".to_string()));
            }
            println!();
        }

        // 关键路径
        if !self.critical_paths.is_empty() {
            println!("🎯 关键路径 ({} 个):", self.critical_paths.len());
            for (i, path) in self.critical_paths.iter().take(5).enumerate() {
                let type_icon = match path.path_type {
                    CriticalPathType::Core => "⭐",
                    CriticalPathType::Hub => "🔗",
                    CriticalPathType::Bridge => "🌉",
                };
                println!("  {} {}: {} (影响: {} 个服务)",
                    type_icon, i + 1, path.critical_service, path.impact_score);
                if path.dependents.len() <= 3 {
                    println!("    依赖服务: {:?}", path.dependents);
                } else {
                    println!("    依赖服务: {} 个服务", path.dependents.len());
                }
            }
            println!();
        }

        // 孤立服务
        if !self.isolated_services.is_empty() {
            println!("🏝️  孤立服务 ({} 个):", self.isolated_services.len());
            for service in &self.isolated_services {
                println!("  - {}", service);
            }
            println!();
        }

        // 高优先级建议
        let high_priority_recommendations: Vec<_> = self.recommendations.iter()
            .filter(|r| matches!(r.priority, RecommendationPriority::Critical | RecommendationPriority::High))
            .collect();

        if !high_priority_recommendations.is_empty() {
            println!("🚨 高优先级建议 ({} 个):", high_priority_recommendations.len());
            for rec in high_priority_recommendations {
                let priority_icon = match rec.priority {
                    RecommendationPriority::Critical => "🔴",
                    RecommendationPriority::High => "🟠",
                    _ => "🟡",
                };
                println!("  {} {}", priority_icon, rec.title);
                println!("    {}", rec.description);
                if rec.affected_services.len() <= 3 {
                    println!("    影响服务: {:?}", rec.affected_services);
                } else {
                    println!("    影响服务: {} 个服务", rec.affected_services.len());
                }
                println!();
            }
        }

        // 统计摘要
        let total_dependencies: usize = self.dependency_graph.values().map(|deps| deps.len()).sum();
        let avg_dependencies = total_dependencies as f64 / self.total_services as f64;

        println!("📈 依赖统计:");
        println!("  总依赖关系: {}", total_dependencies);
        println!("  平均依赖数: {:.1}", avg_dependencies);
        println!("  最大依赖深度: {:?}", self.dependency_levels.values().max());
        println!("  循环依赖数: {}", self.circular_dependencies.len());
        println!("  关键路径数: {}", self.critical_paths.len());
        println!("  孤立服务数: {}", self.isolated_services.len());
    }
}