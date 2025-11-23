// 暂时简化bitable v1模块，只保留基本结构
use openlark_core::config::Config;

pub struct V1 {
    config: Config,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}