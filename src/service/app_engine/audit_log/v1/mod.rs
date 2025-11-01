// audit_log v1 - 审计日志v1版本API
//,
// 包含审计日志的完整功能
use crate::prelude::*;
/// 审计日志v1版本服务
#[derive(Debug, Clone)]
pub struct AuditLogV1Service {
    client: std::sync::Arc<crate::client::LarkClient>,
}
impl AuditLogV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}