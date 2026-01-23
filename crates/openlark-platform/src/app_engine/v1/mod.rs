//! 应用引擎 V1 API
//!
//! 提供应用引擎 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 应用引擎 V1 API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppEngineV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl AppEngineV1 {
    /// 创建新的应用引擎 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }
}
