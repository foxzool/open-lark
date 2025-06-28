use crate::core::config::Config;

// 子模块声明
pub mod audit_log;
pub mod models;
pub mod openapi_log;

// 重新导出服务类型
pub use audit_log::AuditLogService;
pub use openapi_log::OpenapiLogService;

/// 安全合规服务
///
/// 提供飞书安全合规相关功能，包括：
/// - OpenAPI 审计日志
/// - 行为审计日志
pub struct SecurityAndComplianceService {
    pub openapi_log: OpenapiLogService,
    pub audit_log: AuditLogService,
}

impl SecurityAndComplianceService {
    pub fn new(config: Config) -> Self {
        Self {
            openapi_log: OpenapiLogService::new(config.clone()),
            audit_log: AuditLogService::new(config),
        }
    }
}
