use crate::core::{config::Config, trait_system::Service};
use std::sync::Arc;

pub mod v1;

pub struct BitableService {
    config: Config,
    #[allow(dead_code)] // Reserved for future optimizations
    config_arc: Arc<Config>,
    pub v1: v1::V1,
}

impl BitableService {
    pub fn new(config: Config) -> Self {
        let config_arc = Arc::new(config.clone());
        Self {
            config: config.clone(),
            config_arc: config_arc.clone(),
            v1: v1::V1::new(config),
        }
    }

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: Arc<Config>) -> Self {
        Self {
            config: shared.as_ref().clone(),
            config_arc: shared.clone(),
            v1: v1::V1::new(shared.as_ref().clone()),
        }
    }
}

impl Service for BitableService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "bitable"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
