//! card
//!
//! 卡片实体相关 API（cardkit-v1）。

pub mod batch_update;
pub mod create;
pub mod id_convert;
pub mod models;
pub mod settings;
pub mod update;

pub mod element;

use openlark_core::config::Config;

/// card 资源服务
#[derive(Debug, Clone)]
pub struct CardService {
    config: Config,
}

impl CardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建卡片实体
    pub fn create(&self) -> create::CreateCardRequest {
        create::CreateCardRequest::new(self.config.clone())
    }

    /// 全量更新卡片实体
    pub fn update(&self) -> update::UpdateCardRequest {
        update::UpdateCardRequest::new(self.config.clone())
    }

    /// 局部更新卡片实体
    pub fn batch_update(&self) -> batch_update::BatchUpdateCardRequest {
        batch_update::BatchUpdateCardRequest::new(self.config.clone())
    }

    /// 更新卡片实体配置
    pub fn settings(&self) -> settings::UpdateCardSettingsRequest {
        settings::UpdateCardSettingsRequest::new(self.config.clone())
    }

    /// 转换卡片 ID
    pub fn id_convert(&self) -> id_convert::ConvertCardIdRequest {
        id_convert::ConvertCardIdRequest::new(self.config.clone())
    }

    /// 卡片组件相关
    pub fn element(&self) -> element::CardElementService {
        element::CardElementService::new(self.config.clone())
    }
}
