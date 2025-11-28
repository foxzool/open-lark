pub mod aily_session;
pub mod app;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct AilyV1 {
    service: Arc<CommunicationService>,
}

impl AilyV1 {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn aily_session(&self) -> aily_session::AilySession {
        aily_session::AilySession::new(self.service.clone())
    }

    pub fn aily_session_aily_message(&self) -> aily_session::aily_message::AilySessionAilyMessage {
        aily_session::aily_message::AilySessionAilyMessage::new(self.service.clone())
    }

    pub fn aily_session_run(&self) -> aily_session::run::AilySessionRun {
        aily_session::run::AilySessionRun::new(self.service.clone())
    }

    pub fn app_data_asset(&self) -> app::data_asset::AppDataAsset {
        app::data_asset::AppDataAsset::new(self.service.clone())
    }

    pub fn app_data_asset_tag(&self) -> app::data_asset_tag::AppDataAssetTag {
        app::data_asset_tag::AppDataAssetTag::new(self.service.clone())
    }

    pub fn app_knowledge(&self) -> app::knowledge::AppKnowledge {
        app::knowledge::AppKnowledge::new(self.service.clone())
    }

    pub fn app_skill(&self) -> app::skill::AppSkill {
        app::skill::AppSkill::new(self.service.clone())
    }
}