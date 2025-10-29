// audit_log - 审计日志服务
//,
// 提供审计日志相关的功能
use crate::prelude::*;
use crate::service::app_engine::audit_log::v1::AuditLogV1Service;
/// 审计日志服务
#[derive(.*?)]
pub struct AuditLogService {
    /// v1版本API服务
    pub v1: AuditLogV1Service,
}
impl AuditLogService {
    /// 创建新的审计日志服务实例
pub fn new() -> Self {
        Self {
            v1: AuditLogV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;