#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// audit_log - 审计日志服务
//,
// 提供审计日志相关的功能
use openlark_core::prelude::*;
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