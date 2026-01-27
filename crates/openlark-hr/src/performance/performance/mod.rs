/// performance 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod v1;
pub mod v2;

/// performance 项目 v1 版本服务
#[derive(Debug, Clone)]
pub struct PerformanceV1 {
    config: Config,
}

impl PerformanceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// performance 项目 v2 版本服务
#[derive(Debug, Clone)]
pub struct PerformanceV2 {
    config: Config,
}

impl PerformanceV2 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
