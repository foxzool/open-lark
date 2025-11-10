use open_lark_core::config::Config;

pub mod document;
pub mod document_block;
pub mod document_block_descendant;

/// Docx v1版本服务
pub struct V1 {
    /// 文档管理服务
    pub document: document::DocumentService,
    /// 文档块管理服务
    pub document_block: document_block::DocumentBlockService,
}

impl V1 {
    /// 创建Docx v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::V1;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = V1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            document: document::DocumentService::new(config),
            document_block: document_block::DocumentBlockService::new(config),
        }
    }
}