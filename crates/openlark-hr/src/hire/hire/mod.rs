/// hire 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod v1;
pub mod v2;

/// hire 项目 v1 版本服务
#[derive(Debug, Clone)]
pub struct HireV1 {
    config: Config,
}

impl HireV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// hire 项目 v2 版本服务
#[derive(Debug, Clone)]
pub struct HireV2 {
    config: Config,
}

impl HireV2 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
