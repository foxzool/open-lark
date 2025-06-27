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
