/// CCM Docs API Old V1 Docs-API 模块
///
/// 包含所有云文档内容管理相关的API实现

use openlark_core::config::Config;

/// 云文档内容管理服务
#[derive(Debug, Clone)]
pub struct CcmDocsOldV1DocsApi {
    config: Config,
}

impl CcmDocsOldV1DocsApi {
    /// 创建新的云文档内容管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 搜索云文档
    pub fn search_object(&self) -> search_object::SearchObjectRequest {
        search_object::SearchObjectRequest::new(self.config.clone())
    }

    /// 获取元数据
    pub fn meta(&self) -> meta::GetMetadataRequest {
        meta::GetMetadataRequest::new(self.config.clone())
    }
}

// 导出所有API模块
pub mod search_object;
pub mod meta;