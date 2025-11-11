//! CardKit v1 卡片管理服务
//!
//! 提供卡片的创建、更新、配置等管理功能

use config::Config;

pub mod batch_update;
pub mod create;
pub mod settings;
pub mod update;

// 重新导出所有请求和响应类型
pub use batch_update::*;
pub use create::*;
pub use settings::*;
pub use update::*;

/// 卡片管理服务
///
/// 提供卡片的创建、更新、配置等管理功能
pub struct CardService {
    pub config: Config,
}

impl CardService {
    /// 创建新的卡片服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for CardService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}