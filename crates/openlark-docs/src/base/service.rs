use openlark_core::config::Config;

#[derive(Debug, Clone)]
pub struct BaseService {
    config: Config,
}

impl BaseService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    // /// V2版本接口
    // pub fn v2(&self) -> crate::base::base::v2::BaseV2Service {
    //     crate::base::base::v2::BaseV2Service::new(self.config.clone())
    // } // Disabled: base::v2 module is disabled
}
