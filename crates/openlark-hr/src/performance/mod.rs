/// 绩效模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

#[allow(clippy::module_inception)]
pub mod performance;

/// 绩效服务
#[derive(Debug, Clone)]
pub struct Performance {
    config: Config,
}

impl Performance {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 performance 项目 v1 版本服务
    pub fn v1(&self) -> performance::PerformanceV1 {
        performance::PerformanceV1::new(self.config.clone())
    }

    /// 获取 performance 项目 v2 版本服务
    pub fn v2(&self) -> performance::PerformanceV2 {
        performance::PerformanceV2::new(self.config.clone())
    }
}
