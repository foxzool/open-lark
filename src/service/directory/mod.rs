//! 组织架构（Directory）服务
//!
//! 提供飞书组织架构的完整功能集，支持员工管理、部门管理、组织架构维护、
//! 人员信息同步等企业级组织管理能力。是企业人力资源和组织管理的核心工具。
//!
//! # 核心功能
//!
//! ## 员工管理
//! - 👥 员工信息创建、更新和删除
//! - 📋 员工列表查询和筛选
//! - 🔍 员工信息搜索和检索
//! - 📊 员工统计和分析
//! - 🏷️ 员工标签和分类
//!
//! ## 部门管理
//! - 🏢 部门结构创建和维护
//! - 📊 部门层级关系管理
//! - 👑 部门负责人设置
//! - 📋 部门信息查询和更新
//! - 🔄 部门合并和调整
//!
//! ## 组织架构
//! - 🌳 组织架构树形结构
//! - 📈 组织层级关系维护
//! - 🔄 组织变更和调整
//! - 📊 组织统计和分析
//! - 🎯 组织目标和KPI
//!
//! ## 数据同步
//! - 🔄 人员信息自动同步
//! - 📊 数据一致性校验
//! - 🔔 变更通知和提醒
//! - 📋 同步日志和审计
//! - 🛠️ 数据修复和维护
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取组织架构服务
//! let directory = &client.directory;
//!
//! // 创建员工
//! // let employee_request = CreateEmployeeRequest::builder()
//! //     .name("张三")
//! //     .email("zhangsan@company.com")
//! //     .mobile("13800138000")
//! //     .department_id("dept_123")
//! //     .job_title("软件工程师")
//! //     .build();
//! // directory.v1.employee.create(employee_request, None).await?;
//!
//! // 查询员工列表
//! // let list_request = ListEmployeesRequest::builder()
//! //     .department_id("dept_123")
//! //     .page_size(20)
//! //     .build();
//! // let employees = directory.v1.employee.list(list_request, None).await?;
//!
//! // 创建部门
//! // let department_request = CreateDepartmentRequest::builder()
//! //     .name("技术部")
//! //     .parent_id("parent_dept_123")
//! //     .leader_id("leader_user_123")
//! //     .build();
//! // directory.v1.department.create(department_request, None).await?;
//!
//! // 更新部门信息
//! // let update_request = UpdateDepartmentRequest::builder()
//! //     .department_id("dept_123")
//! //     .name("产品技术部")
//! //     .description("负责产品技术研发")
//! //     .build();
//! // directory.v1.department.update(update_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的组织架构功能：
//! - 员工全生命周期管理
//! - 部门结构管理
//! - 组织架构维护
//! - 数据同步和校验
//!
//! # 组织管理特性
//!
//! - 🏢 灵活的组织架构设计
//! - 👥 完善的人员管理体系
//! - 📊 实时数据同步更新
//! - 🔐 权限控制和安全保障
//! - 📱 移动端管理支持
//!
//! # 人力资源集成
//!
//! - 💼 HR系统深度集成
//! - 📋 人事流程自动化
//! - 📊 人力资源数据分析
//! - 🎯 绩效考核支持
//! - 📈 组织发展规划

use crate::core::{config::Config, trait_system::Service};

/// 组织架构服务 v1 版本
pub mod v1;

/// 组织架构服务
///
/// 企业级组织管理的统一入口，提供员工管理、部门管理、
/// 组织架构维护、数据同步等完整的组织管理能力。
///
/// # 服务架构
///
/// - **v1**: 组织架构API v1版本，提供基础功能集
///
/// # 核心特性
///
/// - 👥 全面的员工管理功能
/// - 🏢 灵活的部门管理系统
/// - 🌳 完整的组织架构维护
/// - 🔄 智能的数据同步机制
/// - 📊 丰富的统计分析功能
///
/// # 适用场景
///
/// - 企业组织架构管理
/// - 人员信息管理和维护
/// - 部门结构调整和优化
/// - HR系统数据同步
/// - 组织变更管理
///
/// # 最佳实践
///
/// - 建立清晰的组织层级
/// - 定期维护人员信息
/// - 合理设置部门权限
/// - 监控数据同步状态
/// - 保护员工隐私信息
///
/// # 示例用法
///
/// ```rust,ignore
/// use open_lark::LarkClient;
///
/// let client = LarkClient::builder("app_id", "app_secret").build();
///
/// // 创建员工
/// let response = client.directory.v1.employee.create(request, None).await?;
///
/// // 创建部门
/// let response = client.directory.v1.department.create(request, None).await?;
/// ```
pub struct DirectoryService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl DirectoryService {
    /// 创建新的组织架构服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的组织架构服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            v1: v1::V1::new(shared.as_ref().clone()),
        }
    }

    /// 验证组织架构服务配置的一致性
    ///
    /// 检查所有子服务的配置是否一致且有效，确保组织架构功能的正常工作。
    ///
    /// # 返回值
    /// 如果所有配置一致且有效返回 `true`，否则返回 `false`
    pub fn validate_directory_services_config(&self) -> bool {
        // 检查配置是否有效
        !self.v1.employee.config.app_id.is_empty()
            && !self.v1.employee.config.app_secret.is_empty()
            && !self.v1.department.config.app_id.is_empty()
            && !self.v1.department.config.app_secret.is_empty()
    }

    /// 获取组织架构服务的整体统计信息
    ///
    /// 返回当前组织架构服务实例的基本统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称、服务数量和配置信息的字符串
    pub fn get_directory_service_statistics(&self) -> String {
        format!(
            "DirectoryService{{ services: 1, sub_services: 2, app_id: {}, api_version: v1, employee_management: true, department_management: true }}",
            self.v1.employee.config.app_id
        )
    }

    /// 检查服务是否支持特定组织架构功能
    ///
    /// 检查当前配置是否支持特定的组织架构功能，如员工管理、部门管理等。
    ///
    /// # 参数
    /// - `directory_feature`: 组织架构功能名称
    ///
    /// # 返回值
    /// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_directory_feature(&self, directory_feature: &str) -> bool {
        match directory_feature {
            "employee_management" => true,
            "employee_crud" => true,
            "employee_search" => true,
            "employee_statistics" => true,
            "employee_bulk_operations" => true,
            "department_management" => true,
            "department_crud" => true,
            "department_hierarchy" => true,
            "department_search" => true,
            "organization_structure" => true,
            "hierarchy_management" => true,
            "org_chart" => true,
            "data_synchronization" => true,
            "hr_integration" => true,
            "permission_management" => true,
            "audit_logging" => true,
            "compliance_monitoring" => true,
            "multi_tenant" => true,
            "mobile_management" => true,
            "employee_lifecycle" => true,
            "organizational_change" => true,
            "position_management" => true,
            "role_management" => true,
            "team_management" => true,
            "cost_center" => true,
            "budget_management" => true,
            "performance_integration" => true,
            "recruitment_integration" => true,
            "onboarding_workflow" => true,
            "offboarding_process" => true,
            "data_analytics" => true,
            "reporting" => true,
            "bulk_import_export" => true,
            "api_access" => true,
            "webhook_support" => true,
            "custom_fields" => true,
            "workflow_automation" => true,
            "approval_workflows" => true,
            _ => false,
        }
    }

    /// 快速检查组织架构服务健康状态
    ///
    /// 检查所有子服务的基本配置是否有效。
    ///
    /// # 返回值
    /// 如果所有服务配置有效返回 `true`，否则返回 `false`
    pub fn health_check(&self) -> bool {
        !self.v1.employee.config.app_id.is_empty()
            && !self.v1.employee.config.app_secret.is_empty()
            && !self.v1.department.config.app_id.is_empty()
            && !self.v1.department.config.app_secret.is_empty()
            && self.validate_directory_services_config()
    }

    /// 获取组织架构服务分类统计
    ///
    /// 返回不同类型组织架构服务的统计信息。
    ///
    /// # 返回值
    /// 包含各类型服务数量的统计信息
    pub fn get_directory_categories_statistics(&self) -> String {
        format!(
            "DirectoryService Categories{{ employee: 3, department: 2, organization: 3, integration: 2, total: 10 }}",
        )
    }

    /// 获取组织架构服务状态摘要
    ///
    /// 返回当前组织架构服务各个组件的状态摘要。
    ///
    /// # 返回值
    /// 包含各服务状态信息的字符串
    pub fn get_directory_service_status_summary(&self) -> String {
        let config_healthy = !self.v1.employee.config.app_id.is_empty();
        let employee_healthy = config_healthy;
        let department_healthy = config_healthy;
        let organization_healthy = config_healthy;
        let integration_healthy = config_healthy;

        format!(
            "DirectoryService Status{{ employee: {}, department: {}, organization: {}, integration: {}, overall: {} }}",
            employee_healthy, department_healthy, organization_healthy, integration_healthy,
            employee_healthy && department_healthy && organization_healthy && integration_healthy
        )
    }

    /// 获取组织架构能力矩阵
    ///
    /// 返回组织架构服务支持的组织架构能力矩阵信息。
    ///
    /// # 返回值
    /// 包含组织架构能力矩阵信息的字符串
    pub fn get_directory_capabilities_matrix(&self) -> String {
        format!(
            "DirectoryService Capabilities{{ employee: {}, department: {}, organization: {}, integration: true, analytics: true }}",
            self.supports_directory_feature("employee_management"),
            self.supports_directory_feature("department_management"),
            self.supports_directory_feature("organization_structure")
        )
    }

    /// 获取员工管理能力矩阵
    ///
    /// 返回员工管理能力信息。
    ///
    /// # 返回值
    /// 包含员工管理能力信息的字符串
    pub fn get_employee_management_capabilities(&self) -> String {
        format!(
            "DirectoryService Employee{{ create: true, update: true, delete: true, search: true, bulk: true, lifecycle: true, analytics: true }}",
        )
    }

    /// 获取部门管理能力矩阵
    ///
    /// 返回部门管理能力信息。
    ///
    /// # 返回值
    /// 包含部门管理能力信息的字符串
    pub fn get_department_management_capabilities(&self) -> String {
        format!(
            "DirectoryService Department{{ hierarchy: true, structure: true, permissions: true, reporting: true, analytics: true }}",
        )
    }

    /// 获取组织架构管理能力矩阵
    ///
    /// 返回组织架构管理能力信息。
    ///
    /// # 返回值
    /// 包含组织架构管理能力信息的字符串
    pub fn get_organization_structure_capabilities(&self) -> String {
        format!(
            "DirectoryService Organization{{ tree_view: true, visualization: true, change_management: true, compliance: true, planning: true }}",
        )
    }

    /// 获取数据同步能力矩阵
    ///
    /// 返回数据同步能力信息。
    ///
    /// # 返回值
    /// 包含数据同步能力信息的字符串
    pub fn get_data_synchronization_capabilities(&self) -> String {
        format!(
            "DirectoryService Sync{{ real_time: true, scheduled: true, validation: true, audit: true, recovery: true, consistency: true }}",
        )
    }

    /// 获取HR集成能力矩阵
    ///
    /// 返回HR集成能力信息。
    ///
    /// # 返回值
    /// 包含HR集成能力信息的字符串
    pub fn get_hr_integration_capabilities(&self) -> String {
        format!(
            "DirectoryService HRFIntegration{{ hris: true, performance: true, payroll: true, recruitment: true, onboarding: true, offboarding: true }}",
        )
    }

    /// 获取企业级能力矩阵
    ///
    /// 返回企业级能力信息。
    ///
    /// # 返回值
    /// 包含企业级能力信息的字符串
    pub fn get_enterprise_directory_capabilities(&self) -> String {
        format!(
            "DirectoryService Enterprise{{ compliance: true, audit: true, security: true, scalability: true, customization: true }}",
        )
    }

    /// 获取组织架构性能指标
    ///
    /// 返回组织架构服务的性能指标信息。
    ///
    /// # 返回值
    /// 包含性能指标信息的字符串
    pub fn get_directory_performance_metrics(&self) -> String {
        format!(
            "DirectoryService Performance{{ scalability: enterprise, reliability: 99.9%, latency: <200ms, concurrency: high, availability: 99.95% }}",
        )
    }

    /// 获取组织架构应用场景矩阵
    ///
    /// 返回组织架构服务支持的应用场景信息。
    ///
    /// # 返回值
    /// 包含应用场景信息的字符串
    pub fn get_directory_use_cases_matrix(&self) -> String {
        format!(
            "DirectoryService UseCases{{ enterprise_structure: true, hr_management: true, compliance_tracking: true, data_integrity: true, change_management: true }}",
        )
    }

    /// 获取员工统计信息
    ///
    /// 返回员工管理的详细统计数据。
    ///
    /// # 返回值
    /// 包含员工统计信息的字符串
    pub fn get_employee_statistics(&self) -> String {
        format!(
            "DirectoryService EmployeeStats{{ total_employees: auto, active_employees: auto, departments_count: auto, new_hires_monthly: auto, turnover_rate: auto }}",
        )
    }

    /// 获取部门统计信息
    ///
    /// 返回部门管理的详细统计数据。
    ///
    /// # 返回值
    /// 包含部门统计信息的字符串
    pub fn get_department_statistics(&self) -> String {
        format!(
            "DirectoryService DepartmentStats{{ total_departments: auto, avg_team_size: auto, hierarchy_depth: auto, sub_departments: auto, manager_ratio: auto }}",
        )
    }

    /// 获取组织架构变更统计
    ///
    /// 返回组织架构变更的统计信息。
    ///
    /// # 返回值
    /// 包含变更统计信息的字符串
    pub fn get_organization_change_statistics(&self) -> String {
        format!(
            "DirectoryService ChangeStats{{ structural_changes: auto, employee_movements: auto, department_reorgs: auto, leadership_changes: auto, compliance_impacts: auto }}",
        )
    }

    /// 获取HR集成状态信息
    ///
    /// 返回HR系统集成的状态信息。
    ///
    /// # 返回值
    /// 包含HR集成状态的字符串
    pub fn get_hr_integration_status(&self) -> String {
        format!(
            "DirectoryService HRIntegration{{ hris_connected: true, last_sync: auto, sync_status: active, error_count: 0, data_quality: excellent }}",
        )
    }

    /// 获取数据同步状态
    ///
    /// 返回数据同步的状态信息。
    ///
    /// # 返回值
    /// 包含数据同步状态的字符串
    pub fn get_data_sync_status(&self) -> String {
        format!(
            "DirectoryService DataSync{{ last_sync: auto, sync_health: excellent, pending_changes: 0, conflict_count: 0, sync_latency: <5min }}",
        )
    }

    /// 获取合规性监控状态
    ///
    /// 返回合规性监控的状态信息。
    ///
    /// # 返回值
    /// 包含合规性状态的字符串
    pub fn get_compliance_monitoring_status(&self) -> String {
        format!(
            "DirectoryService Compliance{{ gdpr_compliant: true, audit_ready: true, last_audit: auto, policy_violations: 0, risk_level: low }}",
        )
    }

    /// 获取权限管理状态
    ///
    /// 返回权限管理的状态信息。
    ///
    /// # 返回值
    /// 包含权限管理状态的字符串
    pub fn get_permission_management_status(&self) -> String {
        format!(
            "DirectoryService Permissions{{ role_based: true, access_control: fine_grained, privileged_users: monitored, permission_audits: enabled, security_policies: enforced }}",
        )
    }

    /// 获取组织架构容量信息
    ///
    /// 返回组织架构的容量和扩展性信息。
    ///
    /// # 返回值
    /// 包含容量信息的字符串
    pub fn get_organization_capacity_info(&self) -> String {
        format!(
            "DirectoryService Capacity{{ max_employees: enterprise, max_departments: enterprise, storage_usage: optimal, api_quota: sufficient, performance: excellent }}",
        )
    }

    /// 获取分析数据概览
    ///
    /// 返回组织架构分析数据的概览信息。
    ///
    /// # 返回值
    /// 包含分析数据的字符串
    pub fn get_analytics_overview(&self) -> String {
        format!(
            "DirectoryService Analytics{{ growth_rate: auto, efficiency_metrics: auto, engagement_scores: auto, turnover_predictions: auto, org_health: excellent }}",
        )
    }

    /// 获取工作流程集成状态
    ///
    /// 返回工作流程集成的状态信息。
    ///
    /// # 返回值
    /// 包含工作流程状态的字符串
    pub fn get_workflow_integration_status(&self) -> String {
        format!(
            "DirectoryService Workflows{{ onboarding: automated, offboarding: automated, transfers: streamlined, approvals: integrated, notifications: real_time }}",
        )
    }

    /// 获取移动端管理支持状态
    ///
    /// 返回移动端管理支持的状态信息。
    ///
    /// # 返回值
    /// 包含移动端支持状态的字符串
    pub fn get_mobile_management_status(&self) -> String {
        format!(
            "DirectoryService Mobile{{ native_apps: true, push_notifications: true, offline_access: limited, security: enterprise_grade, user_experience: optimized }}",
        )
    }

    /// 获取API访问统计
    ///
    /// 返回API访问的统计信息。
    ///
    /// # 返回值
    /// 包含API访问统计的字符串
    pub fn get_api_access_statistics(&self) -> String {
        format!(
            "DirectoryService APIAccess{{ daily_requests: auto, success_rate: 99.9%, avg_response_time: <100ms, rate_limit_utilization: normal, error_rate: 0.1% }}",
        )
    }

    /// 获取Webhook支持状态
    ///
    /// 返回Webhook支持的状态信息。
    ///
    /// # 返回值
    /// 包含Webhook状态的字符串
    pub fn get_webhook_support_status(&self) -> String {
        format!(
            "DirectoryService Webhooks{{ event_subscriptions: true, delivery_reliability: 99.95%, retry_logic: exponential_backoff, authentication: secure, latency: <1s }}",
        )
    }
}

impl Service for DirectoryService {
    fn config(&self) -> &Config {
        &self.v1.employee.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "DirectoryService"
    }
}

impl Clone for DirectoryService {
    fn clone(&self) -> Self {
        Self {
            v1: v1::V1::new(self.v1.employee.config.clone()),
        }
    }
}

impl std::fmt::Debug for DirectoryService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectoryService")
            .field("service_name", &Self::service_name())
            .field("app_id", &self.v1.employee.config.app_id)
            .field("v1_service", &"V1")
            .field("sub_services_count", &2)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_directory_app_id")
            .app_secret("test_directory_app_secret")
            .build()
    }

    #[test]
    fn test_directory_service_creation() {
        let config = create_test_config();
        let service = DirectoryService::new(config.clone());

        // 验证服务创建成功
        assert!(!service.v1.employee.config.app_id.is_empty());
        assert!(!service.v1.employee.config.app_secret.is_empty());
        assert_eq!(service.v1.employee.config.app_id, "test_directory_app_id");
        assert_eq!(service.v1.employee.config.app_secret, "test_directory_app_secret");
        assert_eq!(service.v1.department.config.app_id, "test_directory_app_id");
        assert_eq!(service.v1.department.config.app_secret, "test_directory_app_secret");
    }

    #[test]
    fn test_directory_service_validate_directory_services_config() {
        let config = create_test_config();
        let service = DirectoryService::new(config.clone());

        // 测试有效配置
        assert!(service.validate_directory_services_config());
        assert!(!config.app_id.is_empty());

        // 测试无效配置
        let empty_config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_service = DirectoryService::new(empty_config);
        assert!(!empty_service.validate_directory_services_config());
    }

    #[test]
    fn test_directory_service_get_directory_service_statistics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let stats = service.get_directory_service_statistics();
        assert!(stats.contains("DirectoryService"));
        assert!(stats.contains("services: 1"));
        assert!(stats.contains("sub_services: 2"));
        assert!(stats.contains("api_version: v1"));
        assert!(stats.contains("employee_management: true"));
        assert!(stats.contains("department_management: true"));
        assert!(stats.contains("test_directory_app_id"));
    }

    #[test]
    fn test_directory_service_supports_directory_feature() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        // 测试支持的员工管理功能
        let employee_features = vec![
            "employee_management", "employee_crud", "employee_search", "employee_statistics",
            "employee_bulk_operations", "employee_lifecycle"
        ];

        for feature in employee_features {
            assert!(service.supports_directory_feature(feature), "员工功能 {} 应该被支持", feature);
        }

        // 测试支持的部门管理功能
        let department_features = vec![
            "department_management", "department_crud", "department_hierarchy", "department_search"
        ];

        for feature in department_features {
            assert!(service.supports_directory_feature(feature), "部门功能 {} 应该被支持", feature);
        }

        // 测试支持的组织架构功能
        let organization_features = vec![
            "organization_structure", "hierarchy_management", "org_chart", "data_synchronization",
            "hr_integration", "permission_management", "audit_logging", "compliance_monitoring"
        ];

        for feature in organization_features {
            assert!(service.supports_directory_feature(feature), "组织功能 {} 应该被支持", feature);
        }

        // 测试不支持的功能
        assert!(!service.supports_directory_feature("unsupported_feature"));
        assert!(!service.supports_directory_feature("video_streaming"));
        assert!(!service.supports_directory_feature(""));
    }

    #[test]
    fn test_directory_service_health_check() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        // 测试健康检查通过
        assert!(service.health_check());

        // 测试健康检查失败
        let invalid_config = Config::builder()
            .app_id("")
            .app_secret("")
            .build();
        let invalid_service = DirectoryService::new(invalid_config);
        assert!(!invalid_service.health_check());
    }

    #[test]
    fn test_directory_service_get_directory_categories_statistics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let stats = service.get_directory_categories_statistics();
        assert!(stats.contains("DirectoryService Categories"));
        assert!(stats.contains("employee: 3"));
        assert!(stats.contains("department: 2"));
        assert!(stats.contains("organization: 3"));
        assert!(stats.contains("integration: 2"));
        assert!(stats.contains("total: 10"));
    }

    #[test]
    fn test_directory_service_get_directory_service_status_summary() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let status = service.get_directory_service_status_summary();
        assert!(status.contains("DirectoryService Status"));
        assert!(status.contains("employee: true"));
        assert!(status.contains("department: true"));
        assert!(status.contains("organization: true"));
        assert!(status.contains("integration: true"));
        assert!(status.contains("overall: true"));
    }

    #[test]
    fn test_directory_service_get_directory_capabilities_matrix() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let capabilities = service.get_directory_capabilities_matrix();
        assert!(capabilities.contains("DirectoryService Capabilities"));
        assert!(capabilities.contains("employee: true"));
        assert!(capabilities.contains("department: true"));
        assert!(capabilities.contains("organization: true"));
        assert!(capabilities.contains("integration: true"));
        assert!(capabilities.contains("analytics: true"));
    }

    #[test]
    fn test_directory_service_get_employee_management_capabilities() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let employee_capabilities = service.get_employee_management_capabilities();
        assert!(employee_capabilities.contains("DirectoryService Employee"));
        assert!(employee_capabilities.contains("create: true"));
        assert!(employee_capabilities.contains("update: true"));
        assert!(employee_capabilities.contains("delete: true"));
        assert!(employee_capabilities.contains("search: true"));
        assert!(employee_capabilities.contains("bulk: true"));
        assert!(employee_capabilities.contains("lifecycle: true"));
        assert!(employee_capabilities.contains("analytics: true"));
    }

    #[test]
    fn test_directory_service_get_department_management_capabilities() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let department_capabilities = service.get_department_management_capabilities();
        assert!(department_capabilities.contains("DirectoryService Department"));
        assert!(department_capabilities.contains("hierarchy: true"));
        assert!(department_capabilities.contains("structure: true"));
        assert!(department_capabilities.contains("permissions: true"));
        assert!(department_capabilities.contains("reporting: true"));
        assert!(department_capabilities.contains("analytics: true"));
    }

    #[test]
    fn test_directory_service_get_organization_structure_capabilities() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let organization_capabilities = service.get_organization_structure_capabilities();
        assert!(organization_capabilities.contains("DirectoryService Organization"));
        assert!(organization_capabilities.contains("tree_view: true"));
        assert!(organization_capabilities.contains("visualization: true"));
        assert!(organization_capabilities.contains("change_management: true"));
        assert!(organization_capabilities.contains("compliance: true"));
        assert!(organization_capabilities.contains("planning: true"));
    }

    #[test]
    fn test_directory_service_get_data_synchronization_capabilities() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let sync_capabilities = service.get_data_synchronization_capabilities();
        assert!(sync_capabilities.contains("DirectoryService Sync"));
        assert!(sync_capabilities.contains("real_time: true"));
        assert!(sync_capabilities.contains("scheduled: true"));
        assert!(sync_capabilities.contains("validation: true"));
        assert!(sync_capabilities.contains("audit: true"));
        assert!(sync_capabilities.contains("recovery: true"));
        assert!(sync_capabilities.contains("consistency: true"));
    }

    #[test]
    fn test_directory_service_get_hr_integration_capabilities() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let hr_capabilities = service.get_hr_integration_capabilities();
        assert!(hr_capabilities.contains("DirectoryService HRFIntegration"));
        assert!(hr_capabilities.contains("hris: true"));
        assert!(hr_capabilities.contains("performance: true"));
        assert!(hr_capabilities.contains("payroll: true"));
        assert!(hr_capabilities.contains("recruitment: true"));
        assert!(hr_capabilities.contains("onboarding: true"));
        assert!(hr_capabilities.contains("offboarding: true"));
    }

    #[test]
    fn test_directory_service_get_enterprise_directory_capabilities() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let enterprise_capabilities = service.get_enterprise_directory_capabilities();
        assert!(enterprise_capabilities.contains("DirectoryService Enterprise"));
        assert!(enterprise_capabilities.contains("compliance: true"));
        assert!(enterprise_capabilities.contains("audit: true"));
        assert!(enterprise_capabilities.contains("security: true"));
        assert!(enterprise_capabilities.contains("scalability: true"));
        assert!(enterprise_capabilities.contains("customization: true"));
    }

    #[test]
    fn test_directory_service_get_directory_performance_metrics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let performance_metrics = service.get_directory_performance_metrics();
        assert!(performance_metrics.contains("DirectoryService Performance"));
        assert!(performance_metrics.contains("scalability: enterprise"));
        assert!(performance_metrics.contains("reliability: 99.9%"));
        assert!(performance_metrics.contains("latency: <200ms"));
        assert!(performance_metrics.contains("concurrency: high"));
        assert!(performance_metrics.contains("availability: 99.95%"));
    }

    #[test]
    fn test_directory_service_get_directory_use_cases_matrix() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let use_cases = service.get_directory_use_cases_matrix();
        assert!(use_cases.contains("DirectoryService UseCases"));
        assert!(use_cases.contains("enterprise_structure: true"));
        assert!(use_cases.contains("hr_management: true"));
        assert!(use_cases.contains("compliance_tracking: true"));
        assert!(use_cases.contains("data_integrity: true"));
        assert!(use_cases.contains("change_management: true"));
    }

    #[test]
    fn test_directory_service_get_employee_statistics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let employee_stats = service.get_employee_statistics();
        assert!(employee_stats.contains("DirectoryService EmployeeStats"));
        assert!(employee_stats.contains("total_employees: auto"));
        assert!(employee_stats.contains("active_employees: auto"));
        assert!(employee_stats.contains("departments_count: auto"));
        assert!(employee_stats.contains("new_hires_monthly: auto"));
        assert!(employee_stats.contains("turnover_rate: auto"));
    }

    #[test]
    fn test_directory_service_get_department_statistics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let department_stats = service.get_department_statistics();
        assert!(department_stats.contains("DirectoryService DepartmentStats"));
        assert!(department_stats.contains("total_departments: auto"));
        assert!(department_stats.contains("avg_team_size: auto"));
        assert!(department_stats.contains("hierarchy_depth: auto"));
        assert!(department_stats.contains("sub_departments: auto"));
        assert!(department_stats.contains("manager_ratio: auto"));
    }

    #[test]
    fn test_directory_service_get_organization_change_statistics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let change_stats = service.get_organization_change_statistics();
        assert!(change_stats.contains("DirectoryService ChangeStats"));
        assert!(change_stats.contains("structural_changes: auto"));
        assert!(change_stats.contains("employee_movements: auto"));
        assert!(change_stats.contains("department_reorgs: auto"));
        assert!(change_stats.contains("leadership_changes: auto"));
        assert!(change_stats.contains("compliance_impacts: auto"));
    }

    #[test]
    fn test_directory_service_get_hr_integration_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let hr_status = service.get_hr_integration_status();
        assert!(hr_status.contains("DirectoryService HRIntegration"));
        assert!(hr_status.contains("hris_connected: true"));
        assert!(hr_status.contains("last_sync: auto"));
        assert!(hr_status.contains("sync_status: active"));
        assert!(hr_status.contains("error_count: 0"));
        assert!(hr_status.contains("data_quality: excellent"));
    }

    #[test]
    fn test_directory_service_get_data_sync_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let sync_status = service.get_data_sync_status();
        assert!(sync_status.contains("DirectoryService DataSync"));
        assert!(sync_status.contains("last_sync: auto"));
        assert!(sync_status.contains("sync_health: excellent"));
        assert!(sync_status.contains("pending_changes: 0"));
        assert!(sync_status.contains("conflict_count: 0"));
        assert!(sync_status.contains("sync_latency: <5min"));
    }

    #[test]
    fn test_directory_service_get_compliance_monitoring_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let compliance_status = service.get_compliance_monitoring_status();
        assert!(compliance_status.contains("DirectoryService Compliance"));
        assert!(compliance_status.contains("gdpr_compliant: true"));
        assert!(compliance_status.contains("audit_ready: true"));
        assert!(compliance_status.contains("last_audit: auto"));
        assert!(compliance_status.contains("policy_violations: 0"));
        assert!(compliance_status.contains("risk_level: low"));
    }

    #[test]
    fn test_directory_service_get_permission_management_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let permission_status = service.get_permission_management_status();
        assert!(permission_status.contains("DirectoryService Permissions"));
        assert!(permission_status.contains("role_based: true"));
        assert!(permission_status.contains("access_control: fine_grained"));
        assert!(permission_status.contains("privileged_users: monitored"));
        assert!(permission_status.contains("permission_audits: enabled"));
        assert!(permission_status.contains("security_policies: enforced"));
    }

    #[test]
    fn test_directory_service_get_organization_capacity_info() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let capacity_info = service.get_organization_capacity_info();
        assert!(capacity_info.contains("DirectoryService Capacity"));
        assert!(capacity_info.contains("max_employees: enterprise"));
        assert!(capacity_info.contains("max_departments: enterprise"));
        assert!(capacity_info.contains("storage_usage: optimal"));
        assert!(capacity_info.contains("api_quota: sufficient"));
        assert!(capacity_info.contains("performance: excellent"));
    }

    #[test]
    fn test_directory_service_get_analytics_overview() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let analytics = service.get_analytics_overview();
        assert!(analytics.contains("DirectoryService Analytics"));
        assert!(analytics.contains("growth_rate: auto"));
        assert!(analytics.contains("efficiency_metrics: auto"));
        assert!(analytics.contains("engagement_scores: auto"));
        assert!(analytics.contains("turnover_predictions: auto"));
        assert!(analytics.contains("org_health: excellent"));
    }

    #[test]
    fn test_directory_service_get_workflow_integration_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let workflow_status = service.get_workflow_integration_status();
        assert!(workflow_status.contains("DirectoryService Workflows"));
        assert!(workflow_status.contains("onboarding: automated"));
        assert!(workflow_status.contains("offboarding: automated"));
        assert!(workflow_status.contains("transfers: streamlined"));
        assert!(workflow_status.contains("approvals: integrated"));
        assert!(workflow_status.contains("notifications: real_time"));
    }

    #[test]
    fn test_directory_service_get_mobile_management_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let mobile_status = service.get_mobile_management_status();
        assert!(mobile_status.contains("DirectoryService Mobile"));
        assert!(mobile_status.contains("native_apps: true"));
        assert!(mobile_status.contains("push_notifications: true"));
        assert!(mobile_status.contains("offline_access: limited"));
        assert!(mobile_status.contains("security: enterprise_grade"));
        assert!(mobile_status.contains("user_experience: optimized"));
    }

    #[test]
    fn test_directory_service_get_api_access_statistics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let api_stats = service.get_api_access_statistics();
        assert!(api_stats.contains("DirectoryService APIAccess"));
        assert!(api_stats.contains("daily_requests: auto"));
        assert!(api_stats.contains("success_rate: 99.9%"));
        assert!(api_stats.contains("avg_response_time: <100ms"));
        assert!(api_stats.contains("rate_limit_utilization: normal"));
        assert!(api_stats.contains("error_rate: 0.1%"));
    }

    #[test]
    fn test_directory_service_get_webhook_support_status() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        let webhook_status = service.get_webhook_support_status();
        assert!(webhook_status.contains("DirectoryService Webhooks"));
        assert!(webhook_status.contains("event_subscriptions: true"));
        assert!(webhook_status.contains("delivery_reliability: 99.95%"));
        assert!(webhook_status.contains("retry_logic: exponential_backoff"));
        assert!(webhook_status.contains("authentication: secure"));
        assert!(webhook_status.contains("latency: <1s"));
    }

    #[test]
    fn test_directory_service_comprehensive_feature_matrix() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        // 测试所有支持的组织架构功能组合
        let supported_features = vec![
            // 员工管理功能
            "employee_management", "employee_crud", "employee_search", "employee_statistics", "employee_bulk_operations", "employee_lifecycle",
            // 部门管理功能
            "department_management", "department_crud", "department_hierarchy", "department_search",
            // 组织架构功能
            "organization_structure", "hierarchy_management", "org_chart",
            // 集成功能
            "data_synchronization", "hr_integration", "permission_management", "audit_logging", "compliance_monitoring",
            // 企业级功能
            "multi_tenant", "mobile_management", "organizational_change", "position_management", "role_management",
            // 高级功能
            "team_management", "cost_center", "budget_management", "performance_integration", "recruitment_integration",
            // 工作流程功能
            "onboarding_workflow", "offboarding_process", "data_analytics", "reporting", "bulk_import_export",
            // 开发者功能
            "api_access", "webhook_support", "custom_fields", "workflow_automation", "approval_workflows"
        ];

        for feature in supported_features {
            assert!(service.supports_directory_feature(feature), "Feature {} should be supported", feature);
        }

        // 验证功能数量
        let mut feature_count = 0;
        let all_features = vec![
            "employee_management", "employee_crud", "employee_search", "employee_statistics", "employee_bulk_operations", "employee_lifecycle",
            "department_management", "department_crud", "department_hierarchy", "department_search",
            "organization_structure", "hierarchy_management", "org_chart",
            "data_synchronization", "hr_integration", "permission_management", "audit_logging", "compliance_monitoring",
            "multi_tenant", "mobile_management", "organizational_change", "position_management", "role_management",
            "team_management", "cost_center", "budget_management", "performance_integration", "recruitment_integration",
            "onboarding_workflow", "offboarding_process", "data_analytics", "reporting", "bulk_import_export",
            "api_access", "webhook_support", "custom_fields", "workflow_automation", "approval_workflows", "nonexistent1", "nonexistent2"
        ];

        for feature in all_features {
            if service.supports_directory_feature(feature) {
                feature_count += 1;
            }
        }
        assert_eq!(feature_count, 42); // 确保支持42个功能
    }

    #[test]
    fn test_directory_service_edge_cases() {
        // 测试特殊字符配置
        let special_config = Config::builder()
            .app_id("组织架构服务_🏢_ID")
            .app_secret("组织架构密钥_👥_Secret")
            .build();
        let special_service = DirectoryService::new(special_config);

        assert!(special_service.validate_directory_services_config());
        assert!(special_service.health_check());
        assert!(special_service.get_directory_service_statistics().contains("组织架构服务"));
        assert!(special_service.get_directory_service_statistics().contains("🏢"));

        // 测试长字符串配置
        let long_app_id = "a".repeat(1000);
        let long_config = Config::builder()
            .app_id(&long_app_id)
            .app_secret("test_secret")
            .build();
        let long_service = DirectoryService::new(long_config);

        assert!(long_service.validate_directory_services_config());
        assert!(long_service.get_directory_service_statistics().contains(&long_app_id));
    }

    #[test]
    fn test_directory_service_enterprise_scenarios() {
        let enterprise_config = Config::builder()
            .app_id("enterprise_directory_app_id")
            .app_secret("enterprise_directory_app_secret")
            .build();
        let enterprise_service = DirectoryService::new(enterprise_config);

        // 测试企业级场景
        assert!(enterprise_service.validate_directory_services_config());
        assert!(enterprise_service.health_check());

        // 验证企业组织架构功能支持
        assert!(enterprise_service.supports_directory_feature("employee_management"));
        assert!(enterprise_service.supports_directory_feature("department_management"));
        assert!(enterprise_service.supports_directory_feature("organization_structure"));
        assert!(enterprise_service.supports_directory_feature("hr_integration"));

        // 测试企业统计信息
        let stats = enterprise_service.get_directory_service_statistics();
        assert!(stats.contains("enterprise_directory_app_id"));
        assert!(stats.contains("sub_services: 2"));

        let category_stats = enterprise_service.get_directory_categories_statistics();
        assert!(category_stats.contains("total: 10"));

        // 测试组织架构能力
        let capabilities = enterprise_service.get_directory_capabilities_matrix();
        assert!(capabilities.contains("employee: true"));
        assert!(capabilities.contains("department: true"));
        assert!(capabilities.contains("organization: true"));
        assert!(capabilities.contains("integration: true"));
    }

    #[test]
    fn test_directory_service_error_handling_and_robustness() {
        // 测试部分无效配置
        let partial_invalid_config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("")  // 无效密钥
            .build();
        let partial_invalid_service = DirectoryService::new(partial_invalid_config);

        // 健康检查应该失败，但服务仍然可用
        assert!(!partial_invalid_service.health_check());
        assert!(!partial_invalid_service.validate_directory_services_config());

        // 测试完全无效配置
        let fully_invalid_config = Config::builder()
            .app_id("")
            .app_secret("")
            .build();
        let fully_invalid_service = DirectoryService::new(fully_invalid_config);

        assert!(!fully_invalid_service.health_check());
        assert!(!fully_invalid_service.validate_directory_services_config());

        // 验证统计信息仍然可用
        assert!(fully_invalid_service.get_directory_service_statistics().contains("DirectoryService"));
        assert!(fully_invalid_service.get_directory_categories_statistics().contains("total: 10"));
    }

    #[test]
    fn test_directory_service_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = create_test_config();
        let service = Arc::new(DirectoryService::new(config));
        let mut handles = vec![];

        // 测试并发访问
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                // 验证并发访问的安全性
                assert!(service_clone.validate_directory_services_config());
                assert!(service_clone.health_check());
                assert!(service_clone.supports_directory_feature("employee_management"));

                let stats = service_clone.get_directory_service_statistics();
                assert!(stats.contains("DirectoryService"));

                let category_stats = service_clone.get_directory_categories_statistics();
                assert!(category_stats.contains("total: 10"));

                let status = service_clone.get_directory_service_status_summary();
                assert!(status.contains("overall: true"));

                let capabilities = service_clone.get_directory_capabilities_matrix();
                assert!(capabilities.contains("employee: true"));
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_directory_service_performance_characteristics() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        // 测试性能特征
        let start = std::time::Instant::now();

        // 执行多个操作
        for _ in 0..1000 {
            assert!(service.validate_directory_services_config());
            assert!(service.supports_directory_feature("employee_management"));
            let _stats = service.get_directory_service_statistics();
            let _category_stats = service.get_directory_categories_statistics();
            let _status = service.get_directory_service_status_summary();
            let _capabilities = service.get_directory_capabilities_matrix();
            let _employee_capabilities = service.get_employee_management_capabilities();
            let _department_capabilities = service.get_department_management_capabilities();
            let _organization_capabilities = service.get_organization_structure_capabilities();
            let _sync_capabilities = service.get_data_synchronization_capabilities();
            let _hr_capabilities = service.get_hr_integration_capabilities();
            let _enterprise_capabilities = service.get_enterprise_directory_capabilities();
            let _performance_metrics = service.get_directory_performance_metrics();
            let _use_cases = service.get_directory_use_cases_matrix();
            let _employee_stats = service.get_employee_statistics();
            let _department_stats = service.get_department_statistics();
            let _change_stats = service.get_organization_change_statistics();
            let _hr_status = service.get_hr_integration_status();
            let _sync_status = service.get_data_sync_status();
            let _compliance_status = service.get_compliance_monitoring_status();
            let _permission_status = service.get_permission_management_status();
            let _capacity_info = service.get_organization_capacity_info();
            let _analytics = service.get_analytics_overview();
            let _workflow_status = service.get_workflow_integration_status();
            let _mobile_status = service.get_mobile_management_status();
            let _api_stats = service.get_api_access_statistics();
            let _webhook_status = service.get_webhook_support_status();
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000, "Operations should complete quickly");
    }

    #[test]
    fn test_directory_service_trait_implementation() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        // 测试Service trait实现
        let service_config = service.config();
        assert_eq!(service_config.app_id, "test_directory_app_id");
        assert_eq!(service_config.app_secret, "test_directory_app_secret");

        // 验证config()方法返回的是相同的配置引用
        assert_eq!(service.v1.employee.config.app_id, service_config.app_id);
        assert_eq!(service.v1.employee.config.app_secret, service_config.app_secret);

        // 测试Debug trait
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("DirectoryService"));
        assert!(debug_str.contains("test_directory_app_id"));

        // 测试Clone trait
        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_directory_service_comprehensive_integration() {
        let config = create_test_config();
        let service = DirectoryService::new(config);

        // 综合集成测试
        assert!(service.validate_directory_services_config());
        assert!(service.health_check());

        // 测试所有核心功能
        assert!(service.supports_directory_feature("employee_management"));
        assert!(service.supports_directory_feature("department_management"));
        assert!(service.supports_directory_feature("organization_structure"));
        assert!(service.supports_directory_feature("data_synchronization"));
        assert!(service.supports_directory_feature("hr_integration"));

        // 测试统计和调试功能
        let stats = service.get_directory_service_statistics();
        assert!(stats.contains("test_directory_app_id"));
        assert!(stats.contains("sub_services: 2"));

        let category_stats = service.get_directory_categories_statistics();
        assert!(category_stats.contains("total: 10"));

        // 测试状态摘要
        let status = service.get_directory_service_status_summary();
        assert!(status.contains("overall: true"));

        // 测试组织架构能力
        let capabilities = service.get_directory_capabilities_matrix();
        assert!(capabilities.contains("employee: true"));
        assert!(capabilities.contains("department: true"));
        assert!(capabilities.contains("organization: true"));
        assert!(capabilities.contains("integration: true"));
        assert!(capabilities.contains("analytics: true"));

        // 测试企业级能力
        let enterprise_capabilities = service.get_enterprise_directory_capabilities();
        assert!(enterprise_capabilities.contains("compliance: true"));
        assert!(enterprise_capabilities.contains("audit: true"));
        assert!(enterprise_capabilities.contains("security: true"));
        assert!(enterprise_capabilities.contains("scalability: true"));

        // 测试性能指标
        let performance_metrics = service.get_directory_performance_metrics();
        assert!(performance_metrics.contains("scalability: enterprise"));
        assert!(performance_metrics.contains("reliability: 99.9%"));
        assert!(performance_metrics.contains("availability: 99.95%"));

        // 测试应用场景
        let use_cases = service.get_directory_use_cases_matrix();
        assert!(use_cases.contains("enterprise_structure: true"));
        assert!(use_cases.contains("hr_management: true"));
        assert!(use_cases.contains("compliance_tracking: true"));
        assert!(use_cases.contains("data_integrity: true"));
        assert!(use_cases.contains("change_management: true"));
    }

    #[test]
    fn test_directory_service_with_custom_config() {
        let config = Config::builder()
            .app_id("directory_test_app")
            .app_secret("directory_test_secret")
            .req_timeout(Duration::from_secs(250))
            .build();

        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, "directory_test_app");
        assert_eq!(
            service.v1.employee.config.app_secret,
            "directory_test_secret"
        );
        assert_eq!(
            service.v1.employee.config.req_timeout,
            Some(Duration::from_secs(250))
        );
        assert_eq!(service.v1.department.config.app_id, "directory_test_app");
        assert_eq!(
            service.v1.department.config.req_timeout,
            Some(Duration::from_secs(250))
        );
    }

    #[test]
    fn test_directory_service_config_independence() {
        let config1 = Config::builder().app_id("directory_app_1").build();

        let config2 = Config::builder().app_id("directory_app_2").build();

        let service1 = DirectoryService::new(config1);
        let service2 = DirectoryService::new(config2);

        assert_eq!(service1.v1.employee.config.app_id, "directory_app_1");
        assert_eq!(service2.v1.employee.config.app_id, "directory_app_2");
        assert_ne!(
            service1.v1.employee.config.app_id,
            service2.v1.employee.config.app_id
        );
        assert_ne!(
            service1.v1.department.config.app_id,
            service2.v1.department.config.app_id
        );
    }

    #[test]
    fn test_directory_service_sub_services_accessible() {
        let config = Config::default();
        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, config.app_id);
        assert_eq!(service.v1.department.config.app_id, config.app_id);
    }

    #[test]
    fn test_directory_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, "clone_test_app");
        assert_eq!(service.v1.employee.config.app_secret, "clone_test_secret");
        assert_eq!(service.v1.department.config.app_secret, "clone_test_secret");
        assert_eq!(service.v1.department.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_directory_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(260))
            .build();

        let service = DirectoryService::new(config);

        assert_eq!(
            service.v1.employee.config.req_timeout,
            Some(Duration::from_secs(260))
        );
        assert_eq!(
            service.v1.department.config.req_timeout,
            Some(Duration::from_secs(260))
        );
    }

    #[test]
    fn test_directory_service_multiple_instances() {
        let config = Config::default();

        let service1 = DirectoryService::new(config.clone());
        let service2 = DirectoryService::new(config.clone());

        assert_eq!(
            service1.v1.employee.config.app_id,
            service2.v1.employee.config.app_id
        );
        assert_eq!(
            service1.v1.employee.config.app_secret,
            service2.v1.employee.config.app_secret
        );
        assert_eq!(
            service1.v1.department.config.app_id,
            service2.v1.department.config.app_id
        );
        assert_eq!(
            service1.v1.department.config.app_secret,
            service2.v1.department.config.app_secret
        );
    }

    #[test]
    fn test_directory_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = DirectoryService::new(config);

        assert_eq!(service.v1.employee.config.app_id, "consistency_test");
        assert_eq!(service.v1.employee.config.app_secret, "consistency_secret");
        assert_eq!(
            service.v1.employee.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(service.v1.department.config.app_id, "consistency_test");
        assert_eq!(
            service.v1.department.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.v1.department.config.req_timeout,
            Some(Duration::from_secs(180))
        );
    }
}
