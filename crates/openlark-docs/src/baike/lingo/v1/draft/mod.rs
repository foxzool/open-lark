/// Lingo草稿管理模块
use openlark_core::config::Config;

pub mod create;
pub mod update;

pub use create::{CreateDraftRequest, CreateDraftResp};
pub use update::{UpdateDraftRequest, UpdateDraftResp};

/// Lingo草稿服务
#[derive(Debug, Clone)]
pub struct LingoDraftService {
    config: Config,
}

impl LingoDraftService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
