// HR Suite 服务模块

pub mod hire;
pub mod attendance;
pub mod security;
pub mod ehr;/// HR Suite 服务
pub mod corehr;///
/// 提供完整的人力资源管理解决方案，包括：
/// - 👥 **招聘管理**: 完整的招聘流程管理和人才库维护
/// - ⏰ **考勤管理**: 员工考勤、请假、加班等时间管理
/// - 📊 **HR分析**: 人力资源数据统计和分析报告
///
/// 为企业提供专业的人力资源管理平台，支持多种用工形式和复杂组织架构。

#[derive(Debug)]
pub struct HrSuiteService {
    config: openlark_core::config::Config,
}

impl HrSuiteService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for HrSuiteService {
    fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "hr_suite"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}