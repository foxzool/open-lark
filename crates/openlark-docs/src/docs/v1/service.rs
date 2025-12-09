//! Docs 云文档服务
//!
//! 提供企业云文档管理的统一管理接口。
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_core::config::Config;
//! use openlark_docs::docs::v1::service::DocsService;
//! use openlark_docs::docs::v1::content::get::GetDocsContentParams;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let docs = DocsService::new(config);
//!
//! // 获取云文档内容
//! let params = GetDocsContentParams {
//!     document_token: "doc_token".to_string(),
//! };
//! let content = docs.get_content()
//!     .execute(params)
//!     .await?;
//! ```

use openlark_core::config::Config;

// 导入所有API请求类型
use super::content::get::GetDocsContentRequest;

/// Docs 云文档服务
pub struct DocsService {
    config: Config,
}

impl DocsService {
    /// 创建新的Docs服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取云文档内容请求构建器
    pub fn get_content(&self) -> GetDocsContentRequest {
        GetDocsContentRequest::new(self.config.clone())
    }
}