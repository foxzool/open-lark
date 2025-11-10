#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Cloud Docs API v1版本
//!
//! 实现企业级文档管理的核心功能，整合所有子服务模块：
//! - 文档管理和云盘操作
//! - 评论系统和互动功能
//! - 权限管理和访问控制
//! - 知识库和表格操作
//! - AI助手和白板功能

use open_lark_core::config::Config;

/// Cloud Docs服务 v1版本
///
/// 提供统一的入口点来访问所有云文档相关的功能模块。
/// 整合了文档、云盘、评论、权限、知识库、表格、白板等核心服务。
#[derive(Debug, Clone)]
pub struct CloudDocsServiceV1 {
    pub config: Config,
    /// 云盘服务 - 文件和文件夹管理
    pub drive: DriveServiceV1,
    /// 评论服务 - 文档评论和互动
    pub comments: CommentsService,
    /// 知识库服务
    pub wiki: WikiServiceV2,
    /// 表格服务
    pub sheets: SheetsServiceV3,
    /// 白板服务
    pub board: BoardServiceV1,
    /// AI助手服务
    pub assistant: AssistantServiceV1,
}

impl CloudDocsServiceV1 {
    /// 创建Cloud Docs v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::CloudDocsServiceV1;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = CloudDocsServiceV1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            drive: DriveServiceV1::new(config.clone()),
            comments: CommentsService::new(config.clone()),
            wiki: WikiServiceV2::new(config.clone()),
            sheets: SheetsServiceV3::new(config.clone()),
            board: BoardServiceV1::new(config.clone()),
            assistant: AssistantServiceV1::new(config),
        }
    }
}

// ==================== 服务类型定义 ====================

/// 云盘服务 v1
#[derive(Debug, Clone)]
pub struct DriveServiceV1 {
    pub config: Config,
}

impl DriveServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 评论服务
#[derive(Debug, Clone)]
pub struct CommentsService {
    pub config: Config,
}

impl CommentsService {
    /// 创建新的评论服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::CommentsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = CommentsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 知识库服务 v2
#[derive(Debug, Clone)]
pub struct WikiServiceV2 {
    pub config: Config,
}

impl WikiServiceV2 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 表格服务 v3
#[derive(Debug, Clone)]
pub struct SheetsServiceV3 {
    pub config: Config,
}

impl SheetsServiceV3 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 白板服务 v1
#[derive(Debug, Clone)]
pub struct BoardServiceV1 {
    pub config: Config,
}

impl BoardServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// AI助手服务 v1
#[derive(Debug, Clone)]
pub struct AssistantServiceV1 {
    pub config: Config,
}

impl AssistantServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// ==================== 模块导入 ====================

pub mod assistant;
pub mod board;
pub mod comments;
pub mod drive;
// pub mod sheets; // 暂时禁用，需要修复多个编译错误
pub mod wiki;

// 重新导出所有服务类型
pub use wiki::*;
