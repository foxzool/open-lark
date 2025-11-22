#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

//! 旧版文档服务 v2
//!
//! 提供飞书旧版文档v2版本的完整管理功能，包括：
//! - 创建和初始化文档
//! - 获取文档元信息
//! - 获取文档内容（纯文本和富文本）
//! - 编辑文档内容
//! - 获取电子表格元数据

// 重新启用已修复的模块
pub mod batch_update; // ✅ 已修复
pub mod content; // ✅ 已修复
pub mod meta; // ✅ 已修复
              // pub mod sheet_meta;    // 仍然有语法错误，暂时禁用
pub mod create;
pub mod models;
pub mod requests;
pub mod responses;

// 重新导出所有服务类型
pub use batch_update::*;
pub use content::*;
pub use create::*;
pub use meta::*;
// pub use sheet_meta::*;    // 仍然有语法错误，暂时禁用

use openlark_core::config::Config;

/// 旧版文档服务 v2版本
///
/// 提供飞书旧版文档v2版本的完整入口，支持文档的创建、
/// 查询、编辑等基础功能。
#[derive(Clone, Debug)]
pub struct DocV2Service {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
    /// 文档创建服务
    pub create: CreateDocService,
    /// 文档元信息服务
    pub meta: MetaDocService,
    /// 文档内容服务
    pub content: ContentDocService,
    /// 文档编辑服务
    pub batch_update: BatchUpdateDocService,
    // 电子表格元数据服务暂时禁用
    // pub sheet_meta: SheetMetaDocService;
}

impl DocV2Service {
    /// 创建旧版文档 v2 服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::DocV2Service;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DocV2Service::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            create: CreateDocService::new(config.clone()),
            meta: MetaDocService::new(config.clone()),
            content: ContentDocService::new(config.clone()),
            batch_update: BatchUpdateDocService::new(config),
            // sheet_meta: SheetMetaDocService::new(config),  // 暂时禁用
        }
    }
}
