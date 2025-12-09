//! Docx 群公告服务
//!
//! 提供企业群公告管理的统一管理接口。
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_core::config::Config;
//! use openlark_docs::docx::v1::service::DocxService;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let docx = DocxService::new(config);
//!
//! // 获取群公告基本信息
//! let announcement = docx.get_announcement()
//!     .chat_id("chat_id")
//!     .execute()
//!     .await?;
//! ```

use openlark_core::config::Config;

/// Docx 群公告服务
pub struct DocxService {
    config: Config,
}

impl DocxService {
    /// 创建新的Docx服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    // TODO: 添加19个API方法
    // 1. 获取群公告基本信息 - chat.announcement.get
    // 2. 获取群公告所有块 - chat.announcement.block.list
    // 3. 在群公告中创建块 - chat.announcement.block.children.create
    // 4. 批量更新群公告块的内容 - chat.announcement.block.batch_update
    // 5. 获取群公告块的内容 - chat.announcement.block.get
    // 6. 获取所有子块 - chat.announcement.block.children.get
    // 7. 删除群公告中的块 - chat.announcement.block.children.batch_delete
    // 8. 创建文档 - document.create
    // 9. 获取文档基本信息 - document.get
    // 10. 获取文档纯文本内容 - document.raw_content
    // 11. 获取文档所有块 - document.block.list
    // 12. 创建块 - document.block.children.create
    // 13. 批量更新块 - document.block.batch_update
    // 14. 获取块内容 - document.block.get
    // 15. 删除块 - document.block.delete
    // 16. 获取子块 - document.block.children.get
    // 17. 删除子块 - document.block.children.batch_delete
    // 18. 获取图片下载信息 - document.image.get
    // 19. 批量获取图片下载信息 - document.image.batch_get

    // 预留方法位置
}