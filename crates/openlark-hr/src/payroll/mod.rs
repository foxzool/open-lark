/// 薪资模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod payroll;

/// 薪资服务
#[derive(Debug, Clone)]
pub struct Payroll {
    config: Config,
}

impl Payroll {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 payroll 项目 v1 版本服务
    pub fn v1(&self) -> payroll::PayrollV1 {
        payroll::PayrollV1::new(self.config.clone())
    }
}
