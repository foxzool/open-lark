/// 词库管理模块
///
/// 提供词库的查询功能。
use openlark_core::config::Config;

pub mod list;

pub use list::{ListRepoRequest, ListRepoResp};

/// 词库管理服务
#[derive(Debug, Clone)]
pub struct RepoService {
    config: Config,
}

impl RepoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for RepoService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
