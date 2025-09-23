//! 飞书低代码平台（aPass）服务
//!
//! 提供飞书低代码平台的完整功能集，支持席位管理、审计日志、权限管理、
//! 对象操作、函数执行、环境变量、流程管理等企业级低代码开发能力。
//!
//! # 核心功能
//!
//! ## 席位管理
//! - 👥 席位分配和查询管理
//! - 📊 席位活跃度统计
//! - 💰 席位使用计费管理
//! - 🔄 席位资源动态调整
//! - 📈 席位使用趋势分析
//!
//! ## 审计日志
//! - 📋 全方位审计日志查询
//! - 🔍 数据变更轨迹追踪
//! - 📊 审计事件统计分析
//! - 🕒 时间范围灵活筛选
//! - 🔒 安全操作记录管理
//!
//! ## 权限管理
//! - 👑 角色成员权限授权
//! - 📝 记录级权限精细控制
//! - 🔐 用户权限查询验证
//! - 🎯 动态权限分配管理
//! - 🛡️ 安全权限策略执行
//!
//! ## 对象操作
//! - 📊 OQL数据查询语言
//! - 📝 记录CRUD完整操作
//! - 🔄 批量数据处理能力
//! - 🔍 复杂条件查询支持
//! - ⚡ 高性能数据操作
//!
//! ## 函数执行
//! - 🚀 自定义函数调用执行
//! - 📊 函数执行状态监控
//! - 🔄 异步函数处理支持
//! - 📈 函数性能分析
//! - 🛠️ 函数调试和错误处理
//!
//! ## 环境变量
//! - ⚙️ 环境变量查询管理
//! - 🔒 敏感配置安全存储
//! - 🔄 多环境配置切换
//! - 📋 配置版本管理
//! - 🔍 配置依赖关系分析
//!
//! ## 流程管理
//! - 🔄 业务流程发起执行
//! - 👥 人工任务处理分配
//! - 📊 流程状态跟踪监控
//! - ⏰ 流程时效管理
//! - 📈 流程效率分析优化
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
//! // 获取低代码平台服务
//! let apass = &client.apass;
//!
//! // 查询席位信息
//! // let seat_request = QuerySeatRequest::builder()
//! //     .app_id("app_123")
//! //     .start_date("2024-01-01")
//! //     .end_date("2024-01-31")
//! //     .build();
//! // let seats = apass.seat.query(seat_request, None).await?;
//!
//! // 查询审计日志
//! // let audit_request = QueryAuditLogRequest::builder()
//! //     .object_type("record")
//! //     .operation_type("create")
//! //     .start_time("2024-01-01T00:00:00Z")
//! //     .end_time("2024-01-31T23:59:59Z")
//! //     .build();
//! // let logs = apass.audit_log.query(audit_request, None).await?;
//!
//! // 执行OQL查询
//! // let oql_request = ExecuteOQLRequest::builder()
//! //     .query("SELECT * FROM object_123 WHERE status = 'active'")
//! //     .page_size(100)
//! //     .build();
//! // let results = apass.object.execute_oql(oql_request, None).await?;
//!
//! // 调用自定义函数
//! // let function_request = CallFunctionRequest::builder()
//! //     .function_name("calculateRevenue")
//! //     .parameters(serde_json::json!({
//! //         "year": 2024,
//! //         "quarter": 1
//! //     }))
//! //     .build();
//! // let result = apass.function.call(function_request, None).await?;
//!
//! // 发起流程
//! // let flow_request = StartFlowRequest::builder()
//! //     .flow_id("flow_456")
//! //     .input_data(serde_json::json!({
//! //         "applicant": "user_123",
//! //         "amount": 5000
//! //     }))
//! //     .build();
//! // let flow_instance = apass.flow.start(flow_request, None).await?;
//! ```
//!
//! # 低代码特性
//!
//! - 🚀 快速应用开发平台
//! - 📊 可视化数据建模
//! - 🔄 灵活的业务流程设计
//! - 🎨 丰富的UI组件库
//! - ⚡ 高性能运行时引擎
//!
//! # 企业应用
//!
//! - 📋 业务应用快速构建
//! - 🔄 企业流程自动化
//! - 📊 数据管理和分析
//! - 🔗 系统集成和连接
//! - 📈 数字化转型支持

use crate::core::config::Config;

pub mod audit_log;
pub mod environment_variable;
pub mod flow;
pub mod function;
pub mod models;
pub mod object;
pub mod permission;
pub mod seat;

use audit_log::AuditLogService;
use environment_variable::EnvironmentVariableService;
use flow::FlowService;
use function::FunctionService;
use object::ObjectService;
use permission::PermissionService;
use seat::SeatService;

/// 飞书低代码平台服务
///
/// 提供飞书低代码平台（apass）的完整功能，包括：
/// - 席位管理：席位分配查询、席位活跃查询
/// - 审计日志：审计日志查询、数据变更日志查询、审计事件列表
/// - 权限管理：角色成员授权、记录权限用户授权
/// - 对象操作：OQL查询、记录CRUD操作、批量操作
/// - 函数执行：自定义函数调用
/// - 环境变量：环境变量查询和管理
/// - 流程管理：流程发起、人工任务处理
pub struct ApassService {
    /// 席位管理服务
    pub seat: SeatService,
    /// 审计日志服务
    pub audit_log: AuditLogService,
    /// 权限管理服务
    pub permission: PermissionService,
    /// 对象操作服务
    pub object: ObjectService,
    /// 函数执行服务
    pub function: FunctionService,
    /// 环境变量服务
    pub environment_variable: EnvironmentVariableService,
    /// 流程管理服务
    pub flow: FlowService,
}

impl ApassService {
    pub fn new(config: Config) -> Self {
        Self {
            seat: SeatService::new(config.clone()),
            audit_log: AuditLogService::new(config.clone()),
            permission: PermissionService::new(config.clone()),
            object: ObjectService::new(config.clone()),
            function: FunctionService::new(config.clone()),
            environment_variable: EnvironmentVariableService::new(config.clone()),
            flow: FlowService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_apass_service_creation() {
        let config = Config::default();
        let service = ApassService::new(config.clone());

        // Verify all sub-services have the same config
        assert_eq!(service.seat.config.app_id, config.app_id);
        assert_eq!(service.audit_log.config.app_id, config.app_id);
        assert_eq!(service.permission.config.app_id, config.app_id);
        assert_eq!(service.object.config.app_id, config.app_id);
        assert_eq!(service.function.config.app_id, config.app_id);
        assert_eq!(service.environment_variable.config.app_id, config.app_id);
        assert_eq!(service.flow.config.app_id, config.app_id);
    }

    #[test]
    fn test_apass_service_with_custom_config() {
        let config = Config::builder()
            .app_id("apass_test_app")
            .app_secret("apass_test_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = ApassService::new(config.clone());

        // Verify all sub-services got the custom config
        assert_eq!(service.seat.config.app_id, "apass_test_app");
        assert_eq!(service.seat.config.app_secret, "apass_test_secret");
        assert_eq!(
            service.seat.config.req_timeout,
            Some(Duration::from_secs(180))
        );

        assert_eq!(service.audit_log.config.app_id, "apass_test_app");
        assert_eq!(
            service.audit_log.config.req_timeout,
            Some(Duration::from_secs(180))
        );

        assert_eq!(service.permission.config.app_id, "apass_test_app");
        assert_eq!(service.object.config.app_id, "apass_test_app");
        assert_eq!(service.function.config.app_id, "apass_test_app");
        assert_eq!(service.environment_variable.config.app_id, "apass_test_app");
        assert_eq!(service.flow.config.app_id, "apass_test_app");
    }

    #[test]
    fn test_apass_service_config_independence() {
        let config1 = Config::builder().app_id("apass_app_1").build();

        let config2 = Config::builder().app_id("apass_app_2").build();

        let service1 = ApassService::new(config1);
        let service2 = ApassService::new(config2);

        // Verify services are independent
        assert_eq!(service1.seat.config.app_id, "apass_app_1");
        assert_eq!(service2.seat.config.app_id, "apass_app_2");
        assert_ne!(service1.seat.config.app_id, service2.seat.config.app_id);

        assert_eq!(service1.audit_log.config.app_id, "apass_app_1");
        assert_eq!(service2.audit_log.config.app_id, "apass_app_2");
    }

    #[test]
    fn test_apass_service_all_sub_services_accessible() {
        let config = Config::default();
        let service = ApassService::new(config.clone());

        // Test that all sub-services are accessible and properly configured
        assert_eq!(service.seat.config.app_id, config.app_id);
        assert_eq!(service.audit_log.config.app_id, config.app_id);
        assert_eq!(service.permission.config.app_id, config.app_id);
        assert_eq!(service.object.config.app_id, config.app_id);
        assert_eq!(service.function.config.app_id, config.app_id);
        assert_eq!(service.environment_variable.config.app_id, config.app_id);
        assert_eq!(service.flow.config.app_id, config.app_id);
    }

    #[test]
    fn test_apass_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = ApassService::new(config.clone());

        // All sub-services should have the same config values but be independent instances
        let services_configs = [
            &service.seat.config,
            &service.audit_log.config,
            &service.permission.config,
            &service.object.config,
            &service.function.config,
            &service.environment_variable.config,
            &service.flow.config,
        ];

        for service_config in &services_configs {
            assert_eq!(service_config.app_id, "clone_test_app");
            assert_eq!(service_config.app_secret, "clone_test_secret");
        }
    }

    #[test]
    fn test_apass_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(300))
            .build();

        let service = ApassService::new(config);

        // Verify timeout is propagated to all sub-services
        assert_eq!(
            service.seat.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.audit_log.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.permission.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.object.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.function.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.environment_variable.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.flow.config.req_timeout,
            Some(Duration::from_secs(300))
        );
    }

    #[test]
    fn test_apass_service_multiple_instances() {
        let config = Config::default();

        let service1 = ApassService::new(config.clone());
        let service2 = ApassService::new(config.clone());

        // Both services should have the same config values
        assert_eq!(service1.seat.config.app_id, service2.seat.config.app_id);
        assert_eq!(
            service1.audit_log.config.app_id,
            service2.audit_log.config.app_id
        );
        assert_eq!(
            service1.permission.config.app_id,
            service2.permission.config.app_id
        );
        assert_eq!(service1.object.config.app_id, service2.object.config.app_id);
        assert_eq!(
            service1.function.config.app_id,
            service2.function.config.app_id
        );
        assert_eq!(
            service1.environment_variable.config.app_id,
            service2.environment_variable.config.app_id
        );
        assert_eq!(service1.flow.config.app_id, service2.flow.config.app_id);
    }

    #[test]
    fn test_apass_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(120))
            .build();

        let service = ApassService::new(config);

        // Verify all sub-services have consistent configurations
        let configs = [
            &service.seat.config,
            &service.audit_log.config,
            &service.permission.config,
            &service.object.config,
            &service.function.config,
            &service.environment_variable.config,
            &service.flow.config,
        ];

        for config in &configs {
            assert_eq!(config.app_id, "consistency_test");
            assert_eq!(config.app_secret, "consistency_secret");
            assert_eq!(config.req_timeout, Some(Duration::from_secs(120)));
        }

        // Verify all configs are identical
        for i in 1..configs.len() {
            assert_eq!(configs[0].app_id, configs[i].app_id);
            assert_eq!(configs[0].app_secret, configs[i].app_secret);
            assert_eq!(configs[0].req_timeout, configs[i].req_timeout);
        }
    }
}
