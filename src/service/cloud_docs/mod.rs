//! Cloud Docs云文档服务模块
//!
//! 提供飞书企业级云文档管理的完整功能，包括：
//! - 文档创建、编辑、删除、分享
//! - 文档夹管理和组织结构
//! - 版本控制和协作编辑
//! - 评论系统和互动功能
//! - 文档搜索和推荐引擎
//! - 模板管理和快速创建
//! - 权限管理和访问控制

use crate::core::config::Config;

/// Cloud Docs服务
#[derive(Debug, Clone)]
pub struct CloudDocsService {
    pub config: Config,
    pub v1: v1::CloudDocsServiceV1,
}

impl CloudDocsService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::CloudDocsServiceV1::new(config),
        }
    }
}

pub mod v1;

// 重新导出所有模块和类型
pub use v1::*;

// 为了向后兼容，保留旧的类型别名
pub type ClouddocsService = CloudDocsService;
pub type ServiceType = CloudDocsService;
