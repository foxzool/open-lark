//! 飞书卡片 v1 API
//!
//! 提供完整的卡片管理功能，包括卡片的创建、更新、配置等

use open_lark_core::config::Config;

pub mod card;
pub mod card_element;
pub mod models;

// 重新导出所有公共类型
pub use models::*;
pub use card::*;
pub use card_element::*;

/// 飞书卡片 v1 API 服务
pub struct V1 {
    /// 卡片管理
    pub card: card::CardService,
    /// 组件管理
    pub card_element: card_element::CardElementService,
}

impl V1 {
    /// 创建新的v1服务实例
    pub fn new(config: Config) -> Self {
        Self {
            card: card::CardService::new(config.clone()),
            card_element: card_element::CardElementService::new(config),
        }
    }
}

impl Default for V1 {
    fn default() -> Self {
        Self::new(Config::default())
    }
}