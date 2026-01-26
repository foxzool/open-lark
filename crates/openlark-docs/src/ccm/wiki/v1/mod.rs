/// Wiki V1 API 模块
pub mod node;

// 使用通配符导出所有子模块
pub use node::*;

use openlark_core::config::Config;

/// Wiki V1 知识库服务
#[derive(Clone, Debug)]
pub struct WikiV1Service {
    config: Config,
}

impl WikiV1Service {
    /// 创建新的 Wiki V1 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 搜索 Wiki 请求构建器
    pub fn search_wiki(&self) -> node::search::SearchWikiRequest {
        node::search::SearchWikiRequest::new(self.config.clone())
    }
}
