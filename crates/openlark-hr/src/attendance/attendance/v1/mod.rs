use openlark_core::config::Config;

pub mod approval_info;
pub mod archive_rule;
pub mod file;
pub mod group;
pub mod leave_accrual_record;
pub mod leave_employ_expire_record;
pub mod shift;
pub mod user_approval;
pub mod user_daily_shift;
pub mod user_flow;
pub mod user_setting;
pub mod user_stats_data;
pub mod user_stats_field;
pub mod user_stats_view;
pub mod user_task;
pub mod user_task_remedy;

/// attendance 项目 v1 版本服务
#[derive(Debug, Clone)]
pub struct AttendanceV1 {
    config: Config,
}

impl AttendanceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
