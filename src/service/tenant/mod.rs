pub mod models;
pub mod v2;

use crate::core::config::Config;

/// 企业信息服务
pub struct TenantService {
    /// v2版本API
    pub v2: v2::V2,
}

impl TenantService {
    pub fn new(config: Config) -> Self {
        Self {
            v2: v2::V2::new(config),
        }
    }
}
