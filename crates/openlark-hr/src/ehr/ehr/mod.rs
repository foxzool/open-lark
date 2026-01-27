/// ehr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod v1;

/// ehr 项目 v1 版本服务
#[derive(Debug, Clone)]
pub struct EhrV1 {
    config: Config,
}

impl EhrV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
