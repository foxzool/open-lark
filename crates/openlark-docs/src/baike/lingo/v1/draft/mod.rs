/// Lingo草稿管理模块

use openlark_core::config::Config;

// 导出API实现
pub mod create;
pub mod update;

// 重新导出
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
}