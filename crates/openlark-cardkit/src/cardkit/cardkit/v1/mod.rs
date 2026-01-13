//! cardkit v1

pub mod card;

use openlark_core::config::Config;

/// cardkit v1 服务
#[derive(Debug, Clone)]
pub struct CardkitV1Service {
    config: Config,
}

impl CardkitV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn card(&self) -> card::CardService {
        card::CardService::new(self.config.clone())
    }
}
