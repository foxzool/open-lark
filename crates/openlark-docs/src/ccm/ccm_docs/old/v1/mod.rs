//! CCM Docs API Old V1 模块
//!
//! 包含所有云文档内容管理相关的API实现

use openlark_core::config::Config;

/// CCM Docs Old V1 服务
#[derive(Debug, Clone)]
pub struct CcmDocsOldV1 {
    config: Config,
}

impl CcmDocsOldV1 {
    /// 创建新的CCM Docs Old V1服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取文档内容管理API
    pub fn docs_api(&self) -> crate::ccm::ccm_docs::old::v1::docs_api::CcmDocsOldV1DocsApi {
        crate::ccm::ccm_docs::old::v1::docs_api::CcmDocsOldV1DocsApi::new(self.config.clone())
    }
}

// 导出功能模块
pub mod docs_api;