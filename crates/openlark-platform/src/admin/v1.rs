//! 系统管理 V1 API
//!
//! 提供系统管理 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 系统管理 V1 API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AdminV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl AdminV1 {
    /// 创建新的系统管理 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }
}
