use std::sync::Arc;

use crate::{
    core::config::Config,
    service::attendance::v1::{
        archive_rule::ArchiveRuleService, group::GroupService,
        leave_accrual_record::LeaveAccrualRecordService,
        leave_employ_expire_record::LeaveEmployExpireRecordService, shift::ShiftService,
        user_approval::UserApprovalService, user_daily_shift::UserDailyShiftService,
        user_setting::UserSettingService, user_stats_data::UserStatsDataService,
        user_task::UserTaskService, user_task_remedy::UserTaskRemedyService,
    },
};

pub mod archive_rule;
pub mod group;
pub mod leave_accrual_record;
pub mod leave_employ_expire_record;
pub mod models;
pub mod p2_attendance_user_task_status_change_v1;
pub mod p2_attendance_user_task_updated_v1;
pub mod shift;
pub mod user_approval;
pub mod user_daily_shift;
pub mod user_setting;
pub mod user_stats_data;
pub mod user_task;
pub mod user_task_remedy;

pub struct V1 {
    pub shift: ShiftService,
    pub user_daily_shift: UserDailyShiftService,
    pub group: GroupService,
    pub user_setting: UserSettingService,
    pub user_stats_data: UserStatsDataService,
    pub user_approval: UserApprovalService,
    pub user_task: UserTaskService,
    pub user_task_remedy: UserTaskRemedyService,
    pub archive_rule: ArchiveRuleService,
    pub leave_employ_expire_record: LeaveEmployExpireRecordService,
    pub leave_accrual_record: LeaveAccrualRecordService,
}

impl V1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            shift: ShiftService {
                config: Arc::clone(&config),
            },
            user_daily_shift: UserDailyShiftService {
                config: Arc::clone(&config),
            },
            group: GroupService {
                config: Arc::clone(&config),
            },
            user_setting: UserSettingService {
                config: Arc::clone(&config),
            },
            user_stats_data: UserStatsDataService {
                config: Arc::clone(&config),
            },
            user_approval: UserApprovalService {
                config: Arc::clone(&config),
            },
            user_task: UserTaskService {
                config: Arc::clone(&config),
            },
            user_task_remedy: UserTaskRemedyService {
                config: Arc::clone(&config),
            },
            archive_rule: ArchiveRuleService {
                config: Arc::clone(&config),
            },
            leave_employ_expire_record: LeaveEmployExpireRecordService {
                config: Arc::clone(&config),
            },
            leave_accrual_record: LeaveAccrualRecordService { 
                config: Arc::clone(&config) 
            },
        }
    }
}
