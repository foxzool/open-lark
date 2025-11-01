// audit_log - 审计日志服务
//,
// 提供审计日志相关的功能
use crate::prelude::*;
use crate::service::app_engine::audit_log::v1::AuditLogV1Service;
/// 审计日志服务
#[derive(Debug, Clone)]
pub struct AuditLogService {
}

impl AuditLogService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}