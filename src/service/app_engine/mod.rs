// app_engine - 应用引擎平台服务
//,
// 该模块提供飞书应用引擎平台相关的所有功能，包括：
// - 应用管理（创建、配置、部署）
// - 席位管理（分配、监控、计费）
// - 审计日志（操作日志、数据变更记录）
// - 权限管理（角色权限、记录权限）
// - 应用数据管理
//
// 覆盖37个API接口，是企业应用开发的核心平台功能
use crate::prelude::*;
use crate::service::app_engine::apps::AppsService;
use crate::service::app_engine::seat_management::SeatManagementService;
use crate::service::app_engine::audit_log::AuditLogService;
use crate::service::app_engine::permissions::PermissionsService;
/// 应用引擎平台服务
#[cfg(feature = "app_engine")]
#[derive(.*?)]
pub struct AppEngineService {
    /// 应用管理服务
    pub apps: AppsService,
    /// 席位管理服务
    pub seat_management: SeatManagementService,
    /// 审计日志服务
    pub audit_log: AuditLogService,
    /// 权限管理服务
    pub permissions: PermissionsService,
}
#[cfg(feature = "app_engine")]
impl AppEngineService {
/// 创建新的应用引擎平台服务实例
    pub fn new() -> Self {
Self {
            apps: AppsService::new(client.clone()),
            seat_management: SeatManagementService::new(client.clone()),
            audit_log: AuditLogService::new(client.clone()),
            permissions: PermissionsService::new(client.clone()),
        }
}
}
#[cfg(not(feature = "app_engine"))],
pub struct AppEngineService;
/// 数据模型
pub mod models;
/// 各子模块
pub mod apps;
pub mod seat_management;
pub mod audit_log;
pub mod permissions;