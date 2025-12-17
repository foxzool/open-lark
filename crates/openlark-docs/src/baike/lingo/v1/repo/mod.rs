/// 词库管理模块
///
/// 提供词库的查询功能。
use openlark_core::config::Config;

// 导出具体的API实现
pub mod list;

// 重新导出API函数
pub use list::*;

/// 词库管理服务
#[derive(Debug, Clone)]
pub struct RepoService {
    config: Config,
}

impl RepoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for RepoService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
