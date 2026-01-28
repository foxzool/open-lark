//! aPaaS V1 API
//!
//! 提供 aPaaS V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

pub mod apps;
pub mod market;
pub mod tenants;

/// aPaaS V1 API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ApaasV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl ApaasV1 {
    /// 创建新的 aPaaS V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }
}
