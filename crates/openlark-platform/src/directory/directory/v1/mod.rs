//! 目录服务 V1 API
//!
//! 提供目录服务 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

// 声明所有子模块
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
