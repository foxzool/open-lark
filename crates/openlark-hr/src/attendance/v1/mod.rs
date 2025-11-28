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

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct AttendanceV1 {
    service: Arc<HrService>,
}

impl AttendanceV1 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn approval_info(&self) -> approval_info::ApprovalInfo {
        approval_info::ApprovalInfo::new(self.service.clone())
    }

    pub fn archive_rule(&self) -> archive_rule::ArchiveRule {
        archive_rule::ArchiveRule::new(self.service.clone())
    }

    pub fn file(&self) -> file::File {
        file::File::new(self.service.clone())
    }

    pub fn group(&self) -> group::Group {
        group::Group::new(self.service.clone())
    }

    pub fn leave_accrual_record(&self) -> leave_accrual_record::LeaveAccrualRecord {
        leave_accrual_record::LeaveAccrualRecord::new(self.service.clone())
    }

    pub fn leave_employ_expire_record(&self) -> leave_employ_expire_record::LeaveEmployExpireRecord {
        leave_employ_expire_record::LeaveEmployExpireRecord::new(self.service.clone())
    }

    pub fn shift(&self) -> shift::Shift {
        shift::Shift::new(self.service.clone())
    }

    pub fn user_approval(&self) -> user_approval::UserApproval {
        user_approval::UserApproval::new(self.service.clone())
    }

    pub fn user_daily_shift(&self) -> user_daily_shift::UserDailyShift {
        user_daily_shift::UserDailyShift::new(self.service.clone())
    }

    pub fn user_flow(&self) -> user_flow::UserFlow {
        user_flow::UserFlow::new(self.service.clone())
    }

    pub fn user_setting(&self) -> user_setting::UserSetting {
        user_setting::UserSetting::new(self.service.clone())
    }

    pub fn user_stats_data(&self) -> user_stats_data::UserStatsData {
        user_stats_data::UserStatsData::new(self.service.clone())
    }

    pub fn user_stats_field(&self) -> user_stats_field::UserStatsField {
        user_stats_field::UserStatsField::new(self.service.clone())
    }

    pub fn user_stats_view(&self) -> user_stats_view::UserStatsView {
        user_stats_view::UserStatsView::new(self.service.clone())
    }

    pub fn user_task(&self) -> user_task::UserTask {
        user_task::UserTask::new(self.service.clone())
    }

    pub fn user_task_remedy(&self) -> user_task_remedy::UserTaskRemedy {
        user_task_remedy::UserTaskRemedy::new(self.service.clone())
    }
}