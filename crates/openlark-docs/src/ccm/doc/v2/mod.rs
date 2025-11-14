//! 旧版文档服务 v2
//!
//! 提供飞书旧版文档v2版本的完整管理功能，包括：
//! - 创建和初始化文档
//! - 获取文档元信息
//! - 获取文档内容（纯文本和富文本）
//! - 编辑文档内容
//! - 获取电子表格元数据

pub mod create;
pub mod meta;
pub mod content;
pub mod sheet_meta;
pub mod batch_update;
pub mod models;
pub mod requests;
pub mod responses;

// 重新导出所有服务类型
pub use create::*;
pub use meta::*;
pub use content::*;
pub use sheet_meta::*;
pub use batch_update::*;

use openlark_core::config::Config;

/// 旧版文档服务 v2版本
///
/// 提供飞书旧版文档v2版本的统一入口，支持文档的创建、
/// 查询、编辑等基础功能。
#[derive(Clone)]
pub struct DocV2Service {
    config: Config,
    /// 文档创建服务
    pub create: CreateDocService,
    /// 文档元信息服务
    pub meta: MetaDocService,
    /// 文档内容服务
    pub content: ContentDocService,
    /// 电子表格元数据服务
    pub sheet_meta: SheetMetaDocService,
    /// 文档编辑服务
    pub batch_update: BatchUpdateDocService,
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
            sheet_meta: SheetMetaDocService::new(config.clone()),
            batch_update: BatchUpdateDocService::new(config),
        }
    }
}