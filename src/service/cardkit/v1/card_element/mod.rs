use crate::core::config::Config;

pub mod content;
pub mod create;
pub mod delete;
pub mod patch;
pub mod update;

// 重新导出所有请求和响应类型
pub use create::*;

/// 卡片组件管理服务
///
/// 提供卡片组件的创建、更新、删除等管理功能
pub struct CardElementService {
    pub config: Config,
}

impl CardElementService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
