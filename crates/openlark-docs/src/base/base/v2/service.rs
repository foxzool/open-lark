use openlark_core::config::Config;

#[derive(Debug, Clone)]
pub struct BaseV2Service {
    config: Config,
}

impl BaseV2Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 应用管理
    pub fn app(&self) -> crate::base::base::v2::app::AppService {
        crate::base::base::v2::app::AppService::new(self.config.clone())
    }
}
