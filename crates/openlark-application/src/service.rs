use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationService：应用管理服务的统一入口
///
/// 提供对应用 API v1 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct ApplicationService {
    config: Arc<Config>,
}

impl ApplicationService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::application::application::v1::ApplicationV1 {
        crate::application::application::v1::ApplicationV1::new(self.config.clone())
    }
}
