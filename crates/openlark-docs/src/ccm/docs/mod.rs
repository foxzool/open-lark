/// ccm/docs模块 - 云文档内容管理
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// 云文档内容管理服务
#[derive(Debug, Clone)]
pub struct DocsService {
    config: Config,
}

impl DocsService {
    /// 创建新的云文档内容管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V1版本API
    pub fn v1(&self) -> crate::ccm::docs::v1::DocsV1Service {
        crate::ccm::docs::v1::DocsV1Service::new(self.config.clone())
    }
}

pub mod v1;
pub mod v2;

pub use v1::*;
pub use v2::*;
