//! card.element
//!
//! 卡片组件相关 API（cardkit-v1）。

pub mod content;
pub mod create;
pub mod delete;
pub mod models;
pub mod patch;
pub mod update;

pub use models::*;
pub use create::*;
pub use content::*;
pub use delete::*;
pub use patch::*;
pub use update::*;

use openlark_core::config::Config;

/// card.element 资源服务
#[derive(Debug, Clone)]
pub struct CardElementResource {
    config: Config,
}

/// 兼容历史命名：card.element 服务
pub type CardElementService = CardElementResource;

impl CardElementResource {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建卡片组件
    pub fn create(&self) -> create::CreateCardElementRequestBuilder {
        create::CreateCardElementRequestBuilder::new(self.config.clone())
    }

    /// 更新组件
    pub fn update(&self) -> update::UpdateCardElementRequestBuilder {
        update::UpdateCardElementRequestBuilder::new(self.config.clone())
    }

    /// 补丁组件
    pub fn patch(&self) -> patch::PatchCardElementRequestBuilder {
        patch::PatchCardElementRequestBuilder::new(self.config.clone())
    }

    /// 流式更新文本
    pub fn content(&self) -> content::UpdateCardElementContentRequestBuilder {
        content::UpdateCardElementContentRequestBuilder::new(self.config.clone())
    }

    /// 删除组件
    pub fn delete(&self) -> delete::DeleteCardElementRequestBuilder {
        delete::DeleteCardElementRequestBuilder::new(self.config.clone())
    }
}
