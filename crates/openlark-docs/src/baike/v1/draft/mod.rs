/// 草稿管理模块
///
/// 提供知识库草稿的创建和更新功能。

use openlark_core::config::Config;

// 导出具体的API实现
pub mod create;
pub mod update;

// 重新导出API函数
pub use create::*;
pub use update::*;

/// 草稿管理服务
#[derive(Debug, Clone)]
pub struct DraftService {
    config: Config,
}

impl DraftService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for DraftService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}