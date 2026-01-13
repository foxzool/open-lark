//! CardKit 服务主入口

use std::sync::Arc;

/// CardKit 服务主入口
#[derive(Debug, Clone)]
pub struct CardkitService {
    config: Arc<openlark_core::config::Config>,
}

impl CardkitService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// 访问 cardkit 项目 v1
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::cardkit::cardkit::v1::CardkitV1Service {
        crate::cardkit::cardkit::v1::CardkitV1Service::new((*self.config).clone())
    }
}
