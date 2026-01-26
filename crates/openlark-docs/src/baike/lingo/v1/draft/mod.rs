/// Lingo草稿管理模块
use openlark_core::config::Config;

// 导出API实现
pub mod create;
pub mod update;

// 使用通配符导出所有子模块
pub use create::*;
pub use update::*;

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
