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
