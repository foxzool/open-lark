//! Cardkit 卡片组件API模块
//!
//! 提供飞书卡片组件相关的功能，包括：
//! - 卡片实体的创建、更新、配置管理
//! - 卡片组件的增删改查
//! - 流式文本更新
//! - ID类型转换工具
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::cardkit::{CardkitService, CreateCardRequest};
//!
//! let service = CardkitService::new(config);
//!
//! // 创建卡片
//! let response = service
//!     .create_card_builder()
//!     .card_content(card_content)
//!     .execute(&service)
//!     .await?;
//!
//! println!("创建的卡片ID: {:?}", response.card_id);
//! ```

/// 卡片实体管理
pub mod card;

/// 卡片组件管理
pub mod card_element;

// 重新导出主要类型
pub use card::{
    BatchUpdateCardRequestBuilder, CardService, ConvertCardIdRequestBuilder,
    CreateCardRequestBuilder, UpdateCardRequestBuilder, UpdateCardSettingsRequestBuilder,
};
pub use card_element::{
    CardElementService, CreateCardElementRequestBuilder, DeleteCardElementRequestBuilder,
    PatchCardElementRequestBuilder, UpdateCardElementContentRequestBuilder,
    UpdateCardElementRequestBuilder,
};

/// 卡片组件服务
#[derive(Debug, Clone)]
pub struct CardKitService {
    config: openlark_core::config::Config,
}

impl CardKitService {
    /// 创建新的卡片服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 获取卡片实体管理服务
    pub fn card(&self) -> card::CardService {
        card::CardService::new(self.config.clone())
    }

    /// 获取卡片组件管理服务
    pub fn card_element(&self) -> card_element::CardElementService {
        card_element::CardElementService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardkit_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = CardKitService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_submodules() {
        let config = openlark_core::config::Config::default();
        let service = CardKitService::new(config);

        // 测试子模块访问
        let _card_service = service.card();
        let _card_element_service = service.card_element();
    }
}
