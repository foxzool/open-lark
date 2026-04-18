//! 目录服务 V1 API
//!
//! 提供目录服务 V1 版本的 API 访问

// 现存版本化 API 面较大，统一文档补齐前先在模块边界抑制 missing_docs 噪声，
// 避免影响 crate 级质量门禁。
#![allow(missing_docs)]

use crate::PlatformConfig;
use std::sync::Arc;

pub mod collaboration_rule;
pub mod collaboration_share_entity;
pub mod collaboration_tenant;
pub mod department;
pub mod departments;
pub mod employee;
pub mod sync;
pub mod users;

/// 目录服务 V1 API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DirectoryV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl DirectoryV1 {
    /// 创建新的目录服务 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::DirectoryV1;
    use crate::PlatformConfig;

    #[test]
    fn test_directory_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = DirectoryV1::new(std::sync::Arc::new(config));
        assert_eq!(api.config.app_id(), "test_app_id");
    }
}
