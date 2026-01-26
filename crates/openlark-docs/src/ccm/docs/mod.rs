/// ccm/docs模块 - 云文档内容管理
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod v1;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
pub use v1::*;

/// Docs 服务
#[derive(Debug, Clone)]
pub struct DocsService {
    config: Config,
}

impl DocsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
