//! 卡片组件管理服务
//!
//! 提供卡片组件的创建、更新、删除等管理功能

use config::Config;

pub mod content;
pub mod create;
pub mod delete;
pub mod patch;
pub mod update;

// 重新导出所有请求和响应类型
pub use content::*;
pub use create::*;
pub use delete::*;
pub use patch::*;
pub use update::*;

/// 卡片组件管理服务
///
/// 提供卡片组件的创建、更新、删除等管理功能
pub struct CardElementService {
    pub config: Config,
}

impl CardElementService {
    /// 创建新的组件服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for CardElementService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}