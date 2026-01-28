/// EHR 模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

#[allow(clippy::module_inception)]
pub mod ehr;

/// EHR 服务
#[derive(Debug, Clone)]
pub struct Ehr {
    config: Config,
}

impl Ehr {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 ehr 项目 v1 版本服务
    pub fn v1(&self) -> ehr::EhrV1 {
        ehr::EhrV1::new(self.config.clone())
    }
}
