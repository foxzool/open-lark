// Attendance v1 服务模块

pub mod archive_rule;
pub mod group;
pub mod leave_accrual_record;
pub mod leave_employ_expire_record;
pub mod p2_attendance_user_task_status_change_v1;
pub mod p2_attendance_user_task_updated_v1;
pub mod shift;
pub mod user_approval;
pub mod user_daily_shift;
pub mod user_setting;
pub mod user_stats_data;
pub mod user_task;

/// Attendance 服务
///
/// 提供完整的考勤管理解决方案，包括：
/// - 🕐 **排班管理**: 员工排班、调班、换班等功能
/// - 📊 **考勤记录**: 打卡、请假、外出等考勤数据记录
/// - ⏰ **审批流程**: 请假、加班、调班等审批工作流
/// - 📈 **考勤统计**: 各类考勤数据统计和分析报告
/// - 👥 **设置管理**: 考勤规则、班次配置等系统设置
///
/// 为企业提供专业的考勤管理平台，支持多种考勤方式和复杂的企业管理需求。

#[derive(Debug)]
pub struct AttendanceService {
    config: openlark_core::config::Config,
}

impl AttendanceService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for AttendanceService {
    fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "attendance"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
