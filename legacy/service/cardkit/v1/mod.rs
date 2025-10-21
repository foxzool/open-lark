use crate::core::config::Config;

pub mod card;
pub mod card_element;
pub mod models;

pub use models::*;

/// 飞书卡片 v1 API
pub struct V1 {
    /// 卡片管理
    pub card: card::CardService,
    /// 组件管理  
    pub card_element: card_element::CardElementService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            card: card::CardService::new(config.clone()),
            card_element: card_element::CardElementService::new(config),
        }
    }
}
