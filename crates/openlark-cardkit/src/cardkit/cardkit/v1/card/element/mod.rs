//! card.element
//!
//! 卡片组件相关 API（cardkit-v1）。

pub mod content;
pub mod create;
pub mod delete;
pub mod patch;
pub mod update;

use openlark_core::config::Config;

/// card.element 资源服务
#[derive(Debug, Clone)]
pub struct CardElementService {
    config: Config,
}

impl CardElementService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateCardElementRequest {
        create::CreateCardElementRequest::new(self.config.clone())
    }

    pub fn update(&self) -> update::UpdateCardElementRequest {
        update::UpdateCardElementRequest::new(self.config.clone())
    }

    pub fn patch(&self) -> patch::PatchCardElementRequest {
        patch::PatchCardElementRequest::new(self.config.clone())
    }

    pub fn content(&self) -> content::UpdateCardElementContentRequest {
        content::UpdateCardElementContentRequest::new(self.config.clone())
    }

    pub fn delete(&self) -> delete::DeleteCardElementRequest {
        delete::DeleteCardElementRequest::new(self.config.clone())
    }
}
