/// corehr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod v1;
pub mod v2;

/// corehr 项目 v1 版本服务
#[derive(Debug, Clone)]
pub struct CorehrV1 {
    config: Config,
}

impl CorehrV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// corehr 项目 v2 版本服务
#[derive(Debug, Clone)]
pub struct CorehrV2 {
    config: Config,
}

impl CorehrV2 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
