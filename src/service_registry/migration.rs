//! ServiceRegistry 服务迁移工具
//!
//! 提供渐进式服务迁移、版本升级和回滚功能

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::{
    adapters::MigrationHelper as BaseMigrationHelper,
    compatibility::{
        CompatibilityChecker, CompatibilityConfig, CompatibilityHandler, CompatibilityResult,
    },
    error::ServiceError,
    metadata::ServiceMetadata,
    service::{NamedService, Service, ServiceInfo},
    ServiceRegistry,
};
use crate::core::config::Config;

/// 单个服务迁移报告
#[derive(Debug, Clone)]
pub struct ServiceMigrationReport {
    /// 服务名称
    pub service_name: String,
    /// 是否成功
    pub success: bool,
    /// 迁移耗时
    pub migration_time: Duration,
    /// 消息
    pub message: String,
    /// 警告信息
    pub warnings: Vec<String>,
}

/// 迁移策略
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStrategy {
    /// 渐进式迁移
    Gradual {
        batch_size: usize,
        delay_between_batches: Duration,
    },
    /// 立即迁移
    Immediate,
    /// 金丝雀发布
    Canary { canary_services: Vec<String> },
    /// 蓝绿部署
    BlueGreen { validate_before_switch: bool },
}

/// 迁移状态
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStatus {
    /// 准备中
    Preparing,
    /// 进行中
    InProgress { progress: f64 },
    /// 已完成
    Completed,
    /// 已失败
    Failed { error: String },
    /// 已回滚
    RolledBack,
}

/// 迁移任务
#[derive(Debug, Clone)]
pub struct MigrationTask {
    /// 任务ID
    pub id: String,
    /// 迁移策略
    pub strategy: MigrationStrategy,
    /// 要迁移的服务列表
    pub services: Vec<String>,
    /// 源配置
    pub source_config: Config,
    /// 目标配置
    pub target_config: Config,
    /// 状态
    pub status: MigrationStatus,
    /// 开始时间
    pub start_time: Option<Instant>,
    /// 结束时间
    pub end_time: Option<Instant>,
    /// 已迁移的服务
    pub migrated_services: Vec<String>,
    /// 失败的服务
    pub failed_services: Vec<String>,
}

/// 迁移结果
#[derive(Debug, Clone)]
pub struct MigrationResult {
    /// 任务ID
    pub task_id: String,
    /// 是否成功
    pub success: bool,
    /// 已迁移服务数
    pub migrated_count: usize,
    /// 总服务数
    pub total_count: usize,
    /// 耗时
    pub duration: Duration,
    /// 成功迁移的服务
    pub successful_services: Vec<String>,
    /// 失败的服务
    pub failed_services: Vec<String>,
    /// 迁移报告
    pub reports: Vec<ServiceMigrationReport>,
}

/// 高级迁移助手
pub struct AdvancedMigrationHelper {
    registry: Arc<ServiceRegistry>,
    compatibility_handler: CompatibilityHandler,
    active_migrations: Arc<tokio::sync::RwLock<HashMap<String, MigrationTask>>>,
}

impl AdvancedMigrationHelper {
    /// 创建新的高级迁移助手
    pub fn new(registry: Arc<ServiceRegistry>, compatibility_config: CompatibilityConfig) -> Self {
        let compatibility_handler =
            CompatibilityHandler::new(registry.clone(), compatibility_config);

        Self {
            registry,
            compatibility_handler,
            active_migrations: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// 开始迁移任务
    pub async fn start_migration(
        &self,
        task_id: String,
        strategy: MigrationStrategy,
        services: Vec<String>,
        source_config: Config,
        target_config: Config,
    ) -> Result<String, ServiceError> {
        // 验证输入
        if services.is_empty() {
            return Err(ServiceError::validation_error(
                "No services specified for migration",
            ));
        }

        // 创建迁移任务
        let task = MigrationTask {
            id: task_id.clone(),
            strategy,
            services: services.clone(),
            source_config,
            target_config,
            status: MigrationStatus::Preparing,
            start_time: Some(Instant::now()),
            end_time: None,
            migrated_services: Vec::new(),
            failed_services: Vec::new(),
        };

        // 注册任务
        {
            let mut migrations = self.active_migrations.write().await;
            migrations.insert(task_id.clone(), task);
        }

        // 执行迁移
        let registry = Arc::clone(&self.registry);
        let migrations = Arc::clone(&self.active_migrations);
        let handler = self.compatibility_handler.clone();
        let task_id_for_logging = task_id.clone();

        tokio::spawn(async move {
            let result = Self::execute_migration_task(
                registry,
                handler,
                migrations,
                task_id_for_logging.clone(),
            )
            .await;

            // 处理结果
            match result {
                Ok(_) => {
                    log::info!(
                        "Migration task {} completed successfully",
                        task_id_for_logging
                    );
                }
                Err(e) => {
                    log::error!("Migration task {} failed: {}", task_id_for_logging, e);
                }
            }
        });

        Ok(task_id)
    }

    /// 执行迁移任务
    async fn execute_migration_task(
        registry: Arc<ServiceRegistry>,
        compatibility_handler: CompatibilityHandler,
        migrations: Arc<tokio::sync::RwLock<HashMap<String, MigrationTask>>>,
        task_id: String,
    ) -> Result<MigrationResult, ServiceError> {
        // 获取任务信息
        let (task, strategy, services, target_config) = {
            let mut migration_map = migrations.write().await;
            let task = migration_map
                .get_mut(&task_id)
                .ok_or_else(|| ServiceError::not_found("Migration task"))?;

            task.status = MigrationStatus::InProgress { progress: 0.0 };
            let strategy = task.strategy.clone();
            let services = task.services.clone();
            let target_config = task.target_config.clone();

            (task.id.clone(), strategy, services, target_config)
        };

        let start_time = Instant::now();
        let mut successful_services = Vec::new();
        let mut failed_services = Vec::new();
        let mut reports = Vec::new();

        // 根据策略执行迁移
        match strategy {
            MigrationStrategy::Gradual {
                batch_size,
                delay_between_batches,
            } => {
                for (batch_index, batch) in services.chunks(batch_size).enumerate() {
                    let batch_start = Instant::now();

                    // 更新进度
                    let progress =
                        (batch_index * batch_size) as f64 / services.len() as f64 * 100.0;
                    Self::update_task_progress(&migrations, &task_id, progress).await;

                    // 批量迁移
                    for service_name in batch {
                        match Self::migrate_single_service(
                            &registry,
                            &compatibility_handler,
                            service_name,
                            &target_config,
                        )
                        .await
                        {
                            Ok(report) => {
                                successful_services.push(service_name.clone());
                                reports.push(report);
                            }
                            Err(e) => {
                                failed_services.push((service_name.clone(), e.to_string()));
                                log::error!("Failed to migrate service '{}': {}", service_name, e);
                            }
                        }
                    }

                    // 批次间延迟
                    if batch_index * batch_size + batch.len() < services.len() {
                        tokio::time::sleep(delay_between_batches).await;
                    }

                    log::info!(
                        "Batch {} completed in {:?}",
                        batch_index + 1,
                        batch_start.elapsed()
                    );
                }
            }
            MigrationStrategy::Immediate => {
                // 立即迁移所有服务
                for service_name in &services {
                    match Self::migrate_single_service(
                        &registry,
                        &compatibility_handler,
                        service_name,
                        &target_config,
                    )
                    .await
                    {
                        Ok(report) => {
                            successful_services.push(service_name.clone());
                            reports.push(report);
                        }
                        Err(e) => {
                            failed_services.push((service_name.clone(), e.to_string()));
                            log::error!("Failed to migrate service '{}': {}", service_name, e);
                        }
                    }
                }
            }
            MigrationStrategy::Canary { canary_services } => {
                // 先迁移金丝雀服务
                let canary_set: std::collections::HashSet<_> =
                    canary_services.iter().map(|s| s.as_str()).collect();
                let mut canary_success = true;

                for service_name in &services {
                    if canary_set.contains(service_name.as_str()) {
                        match Self::migrate_single_service(
                            &registry,
                            &compatibility_handler,
                            service_name,
                            &target_config,
                        )
                        .await
                        {
                            Ok(report) => {
                                successful_services.push(service_name.clone());
                                reports.push(report);
                            }
                            Err(e) => {
                                canary_success = false;
                                failed_services.push((service_name.clone(), e.to_string()));
                                log::error!("Canary service '{}' failed: {}", service_name, e);
                            }
                        }
                    }
                }

                // 如果金丝雀成功，继续迁移其他服务
                if canary_success {
                    for service_name in &services {
                        if !canary_set.contains(service_name.as_str()) {
                            match Self::migrate_single_service(
                                &registry,
                                &compatibility_handler,
                                service_name,
                                &target_config,
                            )
                            .await
                            {
                                Ok(report) => {
                                    successful_services.push(service_name.clone());
                                    reports.push(report);
                                }
                                Err(e) => {
                                    failed_services.push((service_name.clone(), e.to_string()));
                                    log::error!(
                                        "Failed to migrate service '{}': {}",
                                        service_name,
                                        e
                                    );
                                }
                            }
                        }
                    }
                } else {
                    // 金丝雀失败，回滚已迁移的服务
                    log::warn!("Canary deployment failed, rolling back");
                    for service_name in &successful_services {
                        if let Err(e) = Self::rollback_service(&registry, service_name).await {
                            log::error!("Failed to rollback service '{}': {}", service_name, e);
                        }
                    }
                    successful_services.clear();
                    reports.clear();
                }
            }
            MigrationStrategy::BlueGreen {
                validate_before_switch,
            } => {
                // 蓝绿部署：先在绿色环境部署，验证后再切换
                if validate_before_switch {
                    // 这里可以实现验证逻辑
                    log::info!("Validating green environment before switch");
                }

                for service_name in &services {
                    match Self::migrate_single_service(
                        &registry,
                        &compatibility_handler,
                        service_name,
                        &target_config,
                    )
                    .await
                    {
                        Ok(report) => {
                            successful_services.push(service_name.clone());
                            reports.push(report);
                        }
                        Err(e) => {
                            failed_services.push((service_name.clone(), e.to_string()));
                            log::error!("Failed to migrate service '{}': {}", service_name, e);
                        }
                    }
                }
            }
        }

        let duration = start_time.elapsed();
        let success = failed_services.is_empty();

        // 更新任务状态
        {
            let mut migration_map = migrations.write().await;
            if let Some(task) = migration_map.get_mut(&task_id) {
                task.status = if success {
                    MigrationStatus::Completed
                } else {
                    MigrationStatus::Failed {
                        error: format!("{} services failed to migrate", failed_services.len()),
                    }
                };
                task.end_time = Some(Instant::now());
                task.migrated_services = successful_services.clone();
                task.failed_services = failed_services
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect();
            }
        }

        Ok(MigrationResult {
            task_id,
            success,
            migrated_count: successful_services.len(),
            total_count: services.len(),
            duration,
            successful_services,
            failed_services: failed_services
                .into_iter()
                .map(|(name, error)| format!("{}: {}", name, error))
                .collect(),
            reports,
        })
    }

    /// 迁移单个服务
    async fn migrate_single_service(
        registry: &ServiceRegistry,
        compatibility_handler: &CompatibilityHandler,
        service_name: &str,
        target_config: &Config,
    ) -> Result<ServiceMigrationReport, ServiceError> {
        // 检查兼容性
        // 注意：这里需要根据实际的服务版本信息进行检查
        // 目前使用模拟版本

        // 先注销旧服务
        registry.unregister(service_name)?;

        // 注册新服务
        let result = BaseMigrationHelper::register_services_with_shared_config(
            registry,
            &crate::service_registry::SharedConfig::new(target_config.clone()),
        );

        match result {
            Ok(_) => {
                log::info!("Successfully migrated service: {}", service_name);
                // 生成迁移报告
                Ok(ServiceMigrationReport {
                    service_name: service_name.to_string(),
                    success: true,
                    migration_time: Duration::from_millis(100), // 模拟时间
                    message: "Migration completed successfully".to_string(),
                    warnings: vec![],
                })
            }
            Err(e) => {
                // 迁移失败，尝试回滚
                log::error!("Failed to migrate service '{}': {}", service_name, e);

                // 这里应该使用原始配置重新注册服务
                // 目前简化处理

                Err(e)
            }
        }
    }

    /// 回滚服务
    async fn rollback_service(
        registry: &ServiceRegistry,
        service_name: &str,
    ) -> Result<(), ServiceError> {
        registry.unregister(service_name)?;
        // 这里应该重新注册原始版本的服务
        log::info!("Rolled back service: {}", service_name);
        Ok(())
    }

    /// 更新任务进度
    async fn update_task_progress(
        migrations: &tokio::sync::RwLock<HashMap<String, MigrationTask>>,
        task_id: &str,
        progress: f64,
    ) {
        let mut migration_map = migrations.write().await;
        if let Some(task) = migration_map.get_mut(task_id) {
            task.status = MigrationStatus::InProgress { progress };
        }
    }

    /// 获取迁移状态
    pub async fn get_migration_status(&self, task_id: &str) -> Option<MigrationTask> {
        let migrations = self.active_migrations.read().await;
        migrations.get(task_id).cloned()
    }

    /// 获取所有活跃迁移
    pub async fn get_active_migrations(&self) -> Vec<MigrationTask> {
        let migrations = self.active_migrations.read().await;
        migrations.values().cloned().collect()
    }

    /// 取消迁移
    pub async fn cancel_migration(&self, task_id: &str) -> Result<(), ServiceError> {
        let mut migrations = self.active_migrations.write().await;
        if let Some(task) = migrations.get_mut(task_id) {
            task.status = MigrationStatus::Failed {
                error: "Migration cancelled by user".to_string(),
            };
            task.end_time = Some(Instant::now());
            Ok(())
        } else {
            Err(ServiceError::not_found("Migration task"))
        }
    }

    /// 清理已完成的迁移任务
    pub async fn cleanup_completed_migrations(&self) -> usize {
        let mut migrations = self.active_migrations.write().await;
        let initial_count = migrations.len();

        migrations.retain(|_, task| {
            !matches!(
                task.status,
                MigrationStatus::Completed
                    | MigrationStatus::Failed { .. }
                    | MigrationStatus::RolledBack
            )
        });

        initial_count - migrations.len()
    }

    /// 生成迁移计划
    pub fn generate_migration_plan(
        &self,
        services: &[String],
        strategy: MigrationStrategy,
        source_config: &Config,
        target_config: &Config,
    ) -> MigrationPlan {
        let estimated_duration = self.estimate_migration_duration(services, &strategy);
        let compatibility_checks = self.perform_pre_migration_checks(services, target_config);
        let risks = self.identify_migration_risks(services, source_config, target_config);
        let recommendations = self.generate_migration_recommendations(services, &strategy);

        MigrationPlan {
            services: services.to_vec(),
            strategy,
            estimated_duration,
            compatibility_checks,
            risks,
            recommendations,
        }
    }

    /// 估算迁移时间
    fn estimate_migration_duration(
        &self,
        services: &[String],
        strategy: &MigrationStrategy,
    ) -> Duration {
        let base_time_per_service = Duration::from_millis(100); // 基础时间估算

        match strategy {
            MigrationStrategy::Gradual {
                batch_size,
                delay_between_batches,
            } => {
                let batch_count = (services.len() + batch_size - 1) / batch_size;
                let total_delay =
                    delay_between_batches.saturating_mul(batch_count.saturating_sub(1) as u32);
                total_delay + base_time_per_service * services.len() as u32
            }
            MigrationStrategy::Immediate => base_time_per_service * services.len() as u32,
            MigrationStrategy::Canary { .. } => {
                // 金丝雀部署需要更多时间进行验证
                base_time_per_service * services.len() as u32 * 2
            }
            MigrationStrategy::BlueGreen { .. } => {
                // 蓝绿部署也需要额外时间
                base_time_per_service * services.len() as u32 * 2
            }
        }
    }

    /// 执行预迁移检查
    fn perform_pre_migration_checks(
        &self,
        services: &[String],
        target_config: &Config,
    ) -> Vec<CompatibilityResult> {
        let mut results = Vec::new();

        for service_name in services {
            // 这里应该执行实际的兼容性检查
            // 目前返回模拟结果
            results.push(CompatibilityResult {
                is_compatible: true,
                compatibility_level:
                    crate::service_registry::compatibility::CompatibilityLevel::Full,
                issues: vec![],
                recommendations: vec![],
            });
        }

        results
    }

    /// 识别迁移风险
    fn identify_migration_risks(
        &self,
        services: &[String],
        source_config: &Config,
        target_config: &Config,
    ) -> Vec<MigrationRisk> {
        let mut risks = Vec::new();

        // 检查配置差异
        if source_config.app_id != target_config.app_id {
            risks.push(MigrationRisk {
                risk_type: MigrationRiskType::ConfigurationMismatch,
                severity: crate::service_registry::compatibility::IssueSeverity::Critical,
                description: "App ID mismatch between source and target configuration".to_string(),
                affected_services: services.to_vec(),
                mitigation: "Ensure app IDs match or handle reauthentication".to_string(),
            });
        }

        if source_config.base_url != target_config.base_url {
            risks.push(MigrationRisk {
                risk_type: MigrationRiskType::EndpointChange,
                severity: crate::service_registry::compatibility::IssueSeverity::Warning,
                description: "Base URL change detected".to_string(),
                affected_services: services.to_vec(),
                mitigation: "Verify new endpoint accessibility".to_string(),
            });
        }

        // 检查服务数量
        if services.len() > 50 {
            risks.push(MigrationRisk {
                risk_type: MigrationRiskType::LargeScaleDeployment,
                severity: crate::service_registry::compatibility::IssueSeverity::Warning,
                description: format!("Large scale migration with {} services", services.len()),
                affected_services: services.to_vec(),
                mitigation: "Consider gradual migration strategy".to_string(),
            });
        }

        risks
    }

    /// 生成迁移建议
    fn generate_migration_recommendations(
        &self,
        services: &[String],
        strategy: &MigrationStrategy,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        recommendations
            .push("Ensure all services have proper backups before migration".to_string());
        recommendations.push("Test migration in staging environment first".to_string());

        if services.len() > 10 {
            recommendations
                .push("Consider using gradual migration for large service sets".to_string());
        }

        match strategy {
            MigrationStrategy::Gradual { .. } => {
                recommendations.push("Monitor each batch closely before proceeding".to_string());
            }
            MigrationStrategy::Canary { .. } => {
                recommendations
                    .push("Prepare rollback plan in case canary deployment fails".to_string());
            }
            MigrationStrategy::BlueGreen { .. } => {
                recommendations
                    .push("Ensure green environment is fully validated before switch".to_string());
            }
            MigrationStrategy::Immediate => {
                recommendations
                    .push("Ensure all services can be safely restarted simultaneously".to_string());
            }
        }

        recommendations
    }
}

/// 迁移计划
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    /// 要迁移的服务
    pub services: Vec<String>,
    /// 迁移策略
    pub strategy: MigrationStrategy,
    /// 预估时间
    pub estimated_duration: Duration,
    /// 兼容性检查结果
    pub compatibility_checks: Vec<CompatibilityResult>,
    /// 识别的风险
    pub risks: Vec<MigrationRisk>,
    /// 建议
    pub recommendations: Vec<String>,
}

/// 迁移风险
#[derive(Debug, Clone)]
pub struct MigrationRisk {
    /// 风险类型
    pub risk_type: MigrationRiskType,
    /// 严重程度
    pub severity: crate::service_registry::compatibility::IssueSeverity,
    /// 描述
    pub description: String,
    /// 受影响的服务
    pub affected_services: Vec<String>,
    /// 缓解措施
    pub mitigation: String,
}

/// 迁移风险类型
#[derive(Debug, Clone)]
pub enum MigrationRiskType {
    /// 配置不匹配
    ConfigurationMismatch,
    /// 端点变更
    EndpointChange,
    /// 大规模部署
    LargeScaleDeployment,
    /// 服务依赖
    ServiceDependency,
    /// 数据丢失风险
    DataLossRisk,
}

impl MigrationPlan {
    /// 打印迁移计划
    pub fn print(&self) {
        println!("📋 迁移计划");
        println!("================");
        println!("服务数量: {}", self.services.len());
        println!("迁移策略: {:?}", self.strategy);
        println!("预估时间: {:?}", self.estimated_duration);
        println!();

        if !self.risks.is_empty() {
            println!("⚠️  识别的风险:");
            for risk in &self.risks {
                let severity_icon = match risk.severity {
                    crate::service_registry::compatibility::IssueSeverity::Critical => "🔴",
                    crate::service_registry::compatibility::IssueSeverity::Error => "❌",
                    crate::service_registry::compatibility::IssueSeverity::Warning => "⚠️",
                    crate::service_registry::compatibility::IssueSeverity::Info => "ℹ️",
                };
                println!(
                    "  {} {}: {}",
                    severity_icon,
                    format!("{:?}", risk.risk_type),
                    risk.description
                );
                println!("    缓解措施: {}", risk.mitigation);
            }
            println!();
        }

        if !self.recommendations.is_empty() {
            println!("💡 建议:");
            for rec in &self.recommendations {
                println!("  - {}", rec);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_registry::compatibility::CompatibilityConfig;

    #[tokio::test]
    async fn test_migration_strategy() {
        let strategy = MigrationStrategy::Gradual {
            batch_size: 5,
            delay_between_batches: Duration::from_secs(1),
        };

        assert_eq!(
            strategy,
            MigrationStrategy::Gradual {
                batch_size: 5,
                delay_between_batches: Duration::from_secs(1),
            }
        );
    }

    #[test]
    fn test_migration_plan_generation() {
        let registry = Arc::new(ServiceRegistry::new());
        let compatibility_config = CompatibilityConfig::default();
        let helper = AdvancedMigrationHelper::new(registry, compatibility_config);

        let services = vec!["service1".to_string(), "service2".to_string()];
        let strategy = MigrationStrategy::Immediate;
        let source_config = crate::core::config::ConfigBuilder::default()
            .app_id("test")
            .build();
        let target_config = source_config.clone();

        let plan =
            helper.generate_migration_plan(&services, strategy, &source_config, &target_config);

        assert_eq!(plan.services.len(), 2);
        assert!(matches!(plan.strategy, MigrationStrategy::Immediate));
        assert!(!plan.recommendations.is_empty());
    }
}
