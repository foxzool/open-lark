/// OKR 模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod okr;

/// OKR 服务
#[derive(Debug, Clone)]
pub struct Okr {
    config: Config,
}

impl Okr {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 okr 项目 v1 版本服务
    pub fn v1(&self) -> okr::OkrV1 {
        okr::OkrV1::new(self.config.clone())
    }
}
