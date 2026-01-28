/// 考勤模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

#[allow(clippy::module_inception)]
pub mod attendance;

/// 考勤服务
#[derive(Debug, Clone)]
pub struct Attendance {
    config: Config,
}

impl Attendance {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 attendance 项目 v1 版本服务
    pub fn v1(&self) -> attendance::v1::AttendanceV1 {
        attendance::v1::AttendanceV1::new(self.config.clone())
    }
}
