//! CCM Docs 模块
//!
//! 云文档内容管理相关API实现，包含2个API：
//! - docs_api/search_object: 搜索云文档
//! - docs_api/meta: 获取元数据

use openlark_core::config::Config;

/// CCM Docs 服务
#[derive(Debug, Clone)]
pub struct CcmDocsService {
    config: Config,
}

impl CcmDocsService {
    /// 创建新的CCM Docs服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V1版本API
    pub fn v1(&self) -> CcmDocsV1 {
        CcmDocsV1::new(self.config.clone())
    }

    /// 获取旧版版本API（兼容性保留）
    pub fn old(&self) -> crate::ccm::ccm_docs::old::v1::CcmDocsOldV1 {
        crate::ccm::ccm_docs::old::v1::CcmDocsOldV1::new(self.config.clone())
    }
}

/// CCM Docs V1 API访问器
#[derive(Debug, Clone)]
pub struct CcmDocsV1 {
    config: Config,
}

impl CcmDocsV1 {
    /// 创建新的V1 API访问器实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出版本模块
pub mod old;
pub mod v1;